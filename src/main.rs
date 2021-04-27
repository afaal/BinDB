use bin_pack::BinDB;

#[no_mangle]
static mut DB: BinDB<1024> = BinDB::new();

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let bin_path = bin_pack::get_bin_location(); 
    unsafe {
        DB.init(&bin_path)?; 
        println!("content[0] = {}", DB.content[0]);
        DB.content[0] += 1; 
        DB.commit(&bin_path)?;
    }
    
    Ok(()) 
}
