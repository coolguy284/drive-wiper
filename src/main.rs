use core::fmt;
use std::env::args;
use std::error::Error;
use std::fs::read;
use std::io::ErrorKind;
use std::str::from_utf8;
use std::vec::Vec;

use json::{JsonValue, parse};

#[derive(Debug, Clone)]
struct StatusFormatError;

impl fmt::Display for StatusFormatError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Invalid format for status file")?;
    Ok(())
  }
}

impl Error for StatusFormatError {}

struct WipeStatus {
  drive: String,
  chunk_size: u64,
  chunk_index: u64,
  finished: bool,
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
  
  match data_json {
    JsonValue::Object(_) => (),
    _ => return Err(Box::new(StatusFormatError)),
  };
  
  let Some(drive_str) = data_json["drive"].as_str() else { return Err(Box::new(StatusFormatError)) };
  let Some(chunk_size) = data_json["chunk_size"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  let Some(chunk_index) = data_json["chunk_index"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  let Some(finished) = data_json["finished"].as_bool() else { return Err(Box::new(StatusFormatError)) };
  
  Ok(Some(WipeStatus {
    drive: drive_str.to_owned(),
    chunk_size,
    chunk_index,
    finished,
  }))
}

fn write_prior_status(status: WipeStatus, status_storage_file_path: String) {
  
}

fn wipe_drive(drive: String, status_storage_file_path: String) -> Result<(), Box<dyn Error>> {
  read_prior_status(status_storage_file_path)?;
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
