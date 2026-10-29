use core::fmt;
use std::env::{ArgsOs, args_os};
use std::error::Error;
use std::fs::read;
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::str::from_utf8;
use std::vec::Vec;

use atomic_write_file::AtomicWriteFile;
use json::{JsonValue, object, parse, stringify_pretty};

const STATUS_VERSION: u64 = 1;
const STATUS_JSON_INDENT: u16 = 2;

#[derive(Debug, Clone)]
struct StatusFormatError;

impl fmt::Display for StatusFormatError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Invalid format for status file")?;
    Ok(())
  }
}

impl Error for StatusFormatError {}

#[derive(Debug, Clone)]
struct StatusVersionError {
  expected_ver: u64,
  actual_ver: u64,
}

impl fmt::Display for StatusVersionError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Status version incorrect: expected {}, found {}", self.expected_ver, self.actual_ver)?;
    Ok(())
  }
}

impl Error for StatusVersionError {}

#[derive(Debug, Clone)]
struct ArgDecodeError {
  arg_index: usize,
}

impl fmt::Display for ArgDecodeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Unable to decode argument with index {}", self.arg_index)?;
    Ok(())
  }
}

impl Error for ArgDecodeError {}

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
  
  let Some(version) = data_json["version"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  
  if version != STATUS_VERSION {
    return Err(Box::new(StatusVersionError {
      expected_ver: STATUS_VERSION,
      actual_ver: version,
    }));
  }
  
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

// identical signature to std::fs::write for parity
fn write_atomic<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> std::io::Result<()> {
  let mut file = AtomicWriteFile::open(path)?;
  file.write_all(contents.as_ref())?;
  Ok(())
}

fn write_prior_status(status: WipeStatus, status_storage_file_path: String) -> Result<(), Box<dyn Error>> {
  let data_json = object!{
    version: STATUS_VERSION,
    drive: status.drive,
    chunk_size: status.chunk_size,
    chunk_index: status.chunk_index,
    finished: status.finished,
  };
  
  let data_string = stringify_pretty(data_json, STATUS_JSON_INDENT);
  
  let data_binary = data_string.as_bytes();
  
  match write_atomic(status_storage_file_path, data_binary) {
    Ok(()) => Ok(()),
    Err(error) => return Err(Box::new(error)),
  }
}

fn wipe_drive(drive: String, status_storage_file_path: String) -> Result<(), Box<dyn Error>> {
  let mut status = read_prior_status(status_storage_file_path)?;
  
  if let None = status {
    status = 
  }
  
  Ok(())
}

fn decode_prgm_args(prgm_args: ArgsOs) -> Result<Vec<String>, ArgDecodeError> {
  // https://doc.rust-lang.org/std/result/enum.Result.html#method.from_iter
  
  let converted_args: Result<Vec<String>, ArgDecodeError> =
    prgm_args
      .enumerate()
      .map(|(index, os_str)| match os_str.into_string() {
        Ok(str) => Ok(str),
        Err(_) => Err(ArgDecodeError { arg_index: index }),
      })
      .collect();
  
  return converted_args;
}

fn main() {
  let prgm_args = match decode_prgm_args(args_os()) {
    Ok(prgm_args) => prgm_args,
    Err(error) => {
      println!("Error: {}", error);
      return;
    },
  };
  
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
