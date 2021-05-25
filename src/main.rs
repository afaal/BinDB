use bindb::BinDB;

#[no_mangle]
static mut DB: BinDB<1024> = BinDB::new();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    unsafe {
        DB.init()?.writeable()?; 
        println!("content[0] = {}", DB[0]);
        DB[0] += 2;
    }

    Ok(()) 
}
