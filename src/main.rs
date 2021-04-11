use bin_pack::BinDB;
use std::io::{self, Write}; 

#[no_mangle]
static mut DB: BinDB<1024> = BinDB::new();

fn main() -> Result<(), Box<dyn std::error::Error>>{
    unsafe {
        println!("f_offset = {:x}", DB.f_offset); 
        println!("content[7] = {}", DB.content[7]);
        DB.content[7] += 1; 
        DB.commit()?;
    }
    
    Ok(()) 
}
