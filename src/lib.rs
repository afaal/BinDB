use std::{error::Error, mem, ops::{Deref, DerefMut}, path::{PathBuf}, usize};
use std::{fs, fs::File, io::{Read, Write}}; 
use std::os::unix::fs::PermissionsExt;
use std::ops::{Index, IndexMut};

use memmap::MmapOptions;
pub struct BinDB <const T: usize>  {
    pub f_offset: usize,
    pub content: [u8; T],
    pub file_recreate: bool,
    pub mmap: Option<memmap::MmapMut>
}

impl <const T: usize> BinDB<T> {

    pub const fn new() -> BinDB<T> {
        // add egg 
        let mut content = [0x0; T];
        content[0] = 0xDE;
        content[1] = 0xAD;
        content[2] = 0x13;
        content[3] = 0x33;
        content[4] = 0x37;
        content[5] = 0xBE;
        content[6] = 0xEF;
        
        BinDB {
            f_offset: 0xEFBEADDE,
            content,
            file_recreate: false,
            mmap: None
        }
    }

    pub fn init(&mut self) -> Result<&mut BinDB<T>, Box<dyn Error>> {
        // We call init all the times. So we also need to check whether this has been called before.
        if self.f_offset != 0xEFBEADDE {
            return Ok(self); 
        }

        let path = get_bin_location(); 

        let egg = [0xde,0xad,0x13,0x33,0x37,0xbe,0xef];
        let mut file_dat = read_file(&path.to_owned().to_str().unwrap())?;

        self.f_offset = find_location(&file_dat, &egg)?;

        // Update the f_offset storing the content location in the binary 
        let f_offset_loc = self.f_offset-0x20;
        let offset_slice = self.f_offset.to_le_bytes(); 
        for (i, dat) in offset_slice.iter().enumerate() {
            file_dat[f_offset_loc+i] = *dat; 
        }

        // Remove egg - set all bytes to 0x0
        for i in 0..6 {
            file_dat[self.f_offset+i] = 0x0; 
            self.content[i] = 0x0;
        }

        // Write to file
        let mut mmap = self.recreate_file(&path, &file_dat)?; 
        self.file_recreate = true; 
        self.mmap = Some(mmap); 

        Ok(self)
    }

    pub fn writeable(&mut self) -> Result<(), Box<dyn Error>> {
        self.commit_to_file()?; 
        Ok(())
    }

    // Commit database to disk
    pub fn commit_to_file(&mut self) -> Result<(), Box<dyn Error>> {
        
        // The file has already been recreated and a mmap mapping exists. No reason to recreate the file again
        if self.file_recreate {
            return Ok(())
        }

        let path = get_bin_location(); 

        if self.f_offset == 0xEFBEADDE {
            panic!("The DB hasn't been initalized"); 
        }
        // location of the start of the buffer, we plus 6 so that we don't overwrite the EGG
        // Read file into a buffer
        let mut file_dat = read_file(&path.to_str().unwrap())?;

        // Update binary buffer to contain the updated data. 
        for (i,dat) in self.content.iter().enumerate() {
            file_dat[self.f_offset+i] = *dat;
        }

        // Recreate file and write data to it
        let mmap = self.recreate_file(&path, &file_dat)?; 
        self.file_recreate = true; 
        self.mmap = Some(mmap); 

        Ok(())
    }

    pub fn recreate_file(&mut self, path: &PathBuf, data: &[u8]) -> Result<memmap::MmapMut, Box<dyn Error>> {
        fs::remove_file(&path)?;
        let mut file = std::fs::OpenOptions::new().read(true).write(true).create(true).open(path)?; 
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o770);
        file.set_permissions(perms)?;
        // Write data to file, with the same name as the original (now deleted) file.   
        file.write(&data)?;
        file.flush()?;
        self.file_recreate = true;
        mmap_file(&file, self.f_offset as u64, self.content.len())
    }
}

impl <const T: usize> Index<usize> for BinDB<T> {
    type Output = u8; 

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(mmap) = &self.mmap {
            return &mmap[index]; 
        } else {
            return &self.content[index]; 
        } 
    }
}

impl <const T: usize> IndexMut<usize> for BinDB<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        
        if let Some(mmap) = &mut self.mmap {
            return &mut mmap[index]; 
        } else {
            return &mut self.content[index]; 
        }

    }
}


pub fn create_mmap_file(path: &PathBuf, offset: u64, len:usize) -> Result<memmap::MmapMut, Box<dyn Error>> {
    // we have to open with read+write because that's what we will try to do when mmaping
    // otherwise we will trigger an error
    let mut tmp_file = std::fs::OpenOptions::new().read(true).write(true).create(true).open(path)?; 
    let mut mmap = unsafe { MmapOptions::new().offset(offset).len(len).map_mut(&tmp_file)? }; 
    Ok(mmap)
}

pub fn mmap_file(file: &File, offset: u64, len:usize) -> Result<memmap::MmapMut, Box<dyn Error>> {
    let mut mmap = unsafe { MmapOptions::new().offset(offset).len(len).map_mut(&file)? }; 
    Ok(mmap)
}

// [LINUX ONLY]
// msync - should ensure writes are done to the MMAPed file
pub fn mmap_msync(mmap: &mut memmap::MmapMut) -> Result<(), Box<dyn Error>> {
    unsafe { nix::sys::mman::msync(mmap.as_mut_ptr() as *mut std::ffi::c_void, mmap.len(), nix::sys::mman::MsFlags::MS_SYNC)?; }
    Ok(())
}


pub fn recreate_file(path: &PathBuf, data: &[u8]) -> Result<(), Box<dyn Error>> {
    fs::remove_file(&path)?;
    let mut file = File::create(&path)?;    
    let mut perms = file.metadata()?.permissions();
    perms.set_mode(0o770);
    file.set_permissions(perms)?;
    // Write data to file, with the same name as the original (now deleted) file.   
    file.write(&data)?;
    file.flush()?; 
    Ok(())
}

// [LINUX ONLY]
pub fn get_bin_location() -> PathBuf {
    let path = std::fs::read_link("/proc/self/exe").unwrap();   

    return path; 
}

fn read_file(path: &str) -> Result<Box<[u8]>, Box<dyn Error>> {
    // Read file into a buffer
    let md = fs::metadata(path)?; 
    let mut buff = vec![0;md.len() as usize].into_boxed_slice();
    let mut file = File::open(path)?; 
    file.read(&mut buff)?;
    return Ok(buff);
}

// Returns the start of the egg
fn find_location(buff: &Box<[u8]>, egg: &[u8]) -> Result<usize, Box<dyn Error>> {
    // if buff.contains(x) 
    let location = find_subsequence(&buff, &egg).unwrap();
    Ok(location)
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub fn return_1() -> u32 {
    1
}