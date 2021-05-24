use std::ops::DerefMut;

use bindb::BinDB;

#[no_mangle]
static mut DB: BinDB<1024> = BinDB::new();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bin_path = bindb::get_bin_location(); 
    
    unsafe {
        DB.init(&bin_path)?; 
        println!("content[0] = {}", DB.content[0]);
        DB.content[0] += 1; 
        let mut t = DB.commit_to_file(&bin_path)?;
        t[0] += 2; 
    }

    // ### TEST CODE FOR CREATING MMAPED FILES ### 
    // let p = std::path::PathBuf::from("./test.tmp"); 
    // let mut t = bindb::create_mmap_file(&p)?; 
    
    // t.deref_mut().write_all(b"what is going on here"); 
    // bindb::mmap_msync(&mut t); 

    Ok(()) 
}
