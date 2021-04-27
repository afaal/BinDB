use bindb::BinDB;

#[no_mangle]
static mut DB: BinDB<1024> = BinDB::new();

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let bin_path = bindb::get_bin_location(); 
    unsafe {
        DB.init(&bin_path)?; 
        println!("content[0] = {}", DB.content[0]);
        DB.content[0] += 1; 
        DB.commit_to_file(&bin_path)?;
    }


    let p = std::path::PathBuf::from("./test.tmp"); 

    bindb::create_tmp_file(&p)?; 
    
    Ok(()) 
}
