use std::env::args_os;

use crate::args::decode_prgm_args;
use crate::structs::{WipeParams, WipeStartingParams};
use crate::wiper::wipe_drive;

mod args;
mod consts;
mod errors;
mod io;
mod structs;
mod wiper;

fn main() {
  let prgm_args = match decode_prgm_args(args_os()) {
    Ok(prgm_args) => prgm_args,
    Err(error) => {
      println!("Error: {}", error);
      return;
    },
  };
  
  let start_params;
  
  if prgm_args.len() == 2 {
    start_params = WipeStartingParams::ResumeStatusFile {
      status_storage_file_path: prgm_args[1].clone(),
    };
  } else if prgm_args.len() == 3 {
    start_params = WipeStartingParams::ResumeStatusFileWithDrive {
      drive: prgm_args[1].clone(),
      status_storage_file_path: prgm_args[2].clone(),
    };
  } else if prgm_args.len() == 6 {
    let chunk_size_str = prgm_args[2].clone();
    let start_offset_str = prgm_args[3].clone();
    
    let chunk_size = match chunk_size_str.parse::<u64>() {
      Ok(chunk_size) => chunk_size,
      Err(error) => return println!("Error: {}", error),
    };
    
    let start_offset = match start_offset_str.parse::<u64>() {
      Ok(start_offset) => start_offset,
      Err(error) => return println!("Error: {}", error),
    };
    
    start_params = WipeStartingParams::InitialSetup {
      drive: prgm_args[1].clone(),
      chunk_size,
      status_storage_file_path: prgm_args[4].clone(),
      start_offset: start_offset,
    };
  } else {
    println!("Usage:");
    println!("  New wipe:");
    println!("    drive-wiper <drive> <chunk size> <start offset> <end offset> <status storage file>");
    println!("  Resuming from previous wipe:");
    println!("    drive-wiper <status storage file>");
    println!("    drive-wiper <drive> <status storage file>");
    return;
  }
  
  match wipe_drive(start_params) {
    Ok(()) => println!("Drive wipe successful."),
    Err(err_box) => println!("Error: {}", err_box),
  }
}
