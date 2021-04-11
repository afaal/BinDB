use std::{error::Error, path::PathBuf, usize};
use std::{fs, fs::File, io::{Read, Write}}; 
use std::os::unix::fs::PermissionsExt;
#[derive(Copy, Clone)]
pub struct BinDB <const T: usize> {
    pub f_offset: usize,
    pub content: [u8; T]
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
            content
        }
    }

    // Commit database to disk 
    pub fn commit(&mut self) -> Result<(), Box<dyn Error>> {
        // location of the start of the buffer, we plus 6 so that we don't overwrite the EGG
        let path = get_bin_location().to_str().unwrap().to_owned(); 
        println!("path: {}", path);
        // Read file into a buffer

        let mut file_dat = read_file(&path.as_str())?;
       
        // file offset of content - this is also an egg, but not used atm instead the offset is calculated from
        // the content buffers egg
        if self.f_offset == 0xEFBEADDE {
            println!("Searching for egg..");
            let egg = [0xde,0xad,0x13,0x33,0x37,0xbe,0xef];
            self.f_offset = find_location(&file_dat, &egg)?;
            println!("location: {:x}", self.f_offset);

            // Update the f_offset storing the content location in the binary 
            let f_offset_loc = self.f_offset-0x8;
            let offset_slice = self.f_offset.to_le_bytes(); 
            for (i, dat) in offset_slice.iter().enumerate() {
                file_dat[f_offset_loc+i] = *dat; 
            }
        }
        
        // Remove the original binary and create a new one with execute permissions
        fs::remove_file(&path)?;
        let mut file = File::create(&path.as_str())?;    
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o770);
        file.set_permissions(perms)?;
        println!("Created file"); 

        // Update binary buffer to contain the updated data. 
        for (i,dat) in self.content.iter().enumerate() {
            file_dat[self.f_offset+i] = *dat;
        }

        // Write data to file, with the same name as the original (now deleted) file.
        file.write(&file_dat)?;

        Ok(())
    }   


  

}

  // [LINUX ONLY]
pub fn get_bin_location() -> PathBuf {
    std::fs::read_link("/proc/self/exe").unwrap()
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