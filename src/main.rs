use std::env::args;
use std::error::Error;
use std::fs::read;
use std::io::ErrorKind;
use std::str::from_utf8;
use std::vec::Vec;

use json::parse;

struct WipeStatus {
  
}

fn read_prior_status(status_storage_file_path: String) -> Result<Option<WipeStatus>, Box<dyn Error>> {
  // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
  let data_binary: Vec<u8> = match read(status_storage_file_path) {
    Ok(data) => data,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => return Ok(None),
      _ => return Err(Box::new(error)),
    },
  };
  
  let data_string = match from_utf8(&data_binary) {
    Ok(data) => data,
    Err(error) => return Err(Box::new(error)),
  };
  
  let data_json = match parse(data_string) {
    Ok(data) => data,
    Err(error) => return Err(Box::new(error)),
  };
  
  Ok(Some(WipeStatus {}))
}

fn wipe_drive(drive: String, status_storage_file_path: String) -> Result<(), Box<dyn Error>> {
  Ok(())
}

fn main() {
  // TODO: args might panic if arguments are not valid unicode or smth, do a manual parse instead to make it a result
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
