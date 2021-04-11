use bin_pack::BinDB;
use std::io::{self, Write}; 
// Methods for finding the variable offset
// ------------------------------------------------------
// Use the memory location of the variable, and subtract that from the .data segment map to find the binary offset
//  * Architecture dependant
//  * Requires file I/O

// Use EGG 
//  * Search for the egg in the binary to find the binary offset. 
//  * This doesn't change between compiled binaries, so this could also be written in to the binary, so we only search for the EGG on the first run
//  * We can store serialized data in this blob too, which we can unserialized structs/primitives of the developers choosing
//  * If we maintain one address pointing to the file offset of the buffer, then we don't need the EGG after the first run.

// Problems
//  * We cant modify the binary while it is executing 
//      * Create temporary file, and move it over the previous one
//      * Change the memory protected of the mmap'ed region to allow writing??
//      * Create orphan processes to modify the file? 

// Performance gains
//  * Have a temporary file along side, and just switch copy/overwrite upon commits, that way we won't have to read, and write to entirely new files. 

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
