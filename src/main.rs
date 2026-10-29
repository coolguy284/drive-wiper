use std::env::args;
use std::error::Error;
use std::vec::Vec;

fn wipe_drive(drive: String, status_storage_file_path: String) -> Result<(), Box<dyn Error>> {
  Ok(())
}

fn main() {
  let prgm_args = args().collect::<Vec<String>>();
  
  if prgm_args.len() != 3 {
    println!("Usage:");
    println!("  drive-wiper <drive> <status storage file>");
  } else {
    let drive = prgm_args[1].clone();
    let status_storage_file_path = prgm_args[2].clone();
    
    wipe_drive(drive, status_storage_file_path);
  }
}
