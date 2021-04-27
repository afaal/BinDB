use std::{error::Error, path::{Path, PathBuf}, usize};
use std::{fs, fs::File, io::{Read, Write}}; 
use std::os::unix::fs::PermissionsExt;
#[derive(Copy, Clone)]
pub struct BinDB <const T: usize>  {
    pub f_offset: usize,
    pub content: [u8; T], 
}

// TODO: when deleting a file - the path will become "xxx (deleted)" so upon retriveing the path again, it will no longer be the correct path. 
// We either have to fix the API for retriving the path, or have the path saved throughout the entire lifetime of the program.

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
            content
        }
    }

    pub fn init(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        // We call init all the times. So we also need to check whether this has been called before.
        if self.f_offset != 0xEFBEADDE {
            return Ok(()); 
        }

        let egg = [0xde,0xad,0x13,0x33,0x37,0xbe,0xef];
        let mut file_dat = read_file(&path.to_owned().to_str().unwrap())?;

        self.f_offset = find_location(&file_dat, &egg)?;

        // Update the f_offset storing the content location in the binary 
        let f_offset_loc = self.f_offset-0x8;
        let offset_slice = self.f_offset.to_le_bytes(); 
        for (i, dat) in offset_slice.iter().enumerate() {
            file_dat[f_offset_loc+i] = *dat; 
        }

        // Remove egg 
        for i in 0..6 {
            file_dat[self.f_offset+i] = 0x0; 
            self.content[i] = 0x0;
        }

        // Write to file
        recreate_file(&path, &file_dat)?; 

        Ok(())
    }

    // Commit database to disk
    pub fn commit(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
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
        recreate_file(&path, &file_dat)?; 

        Ok(())
    }
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