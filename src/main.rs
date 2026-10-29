use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
use std::vec::Vec;

struct WipeStatus {
  
}

fn read_prior_status(status_storage_file_path: String) -> Result<Option<WipeStatus>, Box<dyn Error>> {
  // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
  let status_file = match File::open(status_storage_file_path) {
    Ok(file) => file,
    Error(error) => match error.kind() {
      ErrorKind::NotFound => return Ok(None),
    },
  }
  
  Ok(Some(WipeStatus {}))
}

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
    
    match wipe_drive(drive, status_storage_file_path) {
      Ok(()) => println!("Drive wipe successful."),
      Err(err_box) => println!("Error: {}", err_box),
    }
  }
}
