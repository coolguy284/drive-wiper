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
  let status_storage_file_path;
  
  if prgm_args.len() == 2 {
    start_params = None;
    status_storage_file_path = prgm_args[1].clone();
  } else if prgm_args.len() == 3 {
    let drive = prgm_args[1].clone();
    start_params = None;
    status_storage_file_path = prgm_args[2].clone();
  } else if prgm_args.len() == 5 {
    let drive = prgm_args[1].clone();
    let chunk_size_str = prgm_args[2].clone();
    let start_index_str = prgm_args[3].clone();
    status_storage_file_path = prgm_args[4].clone();
    
    let chunk_size = match chunk_size_str.parse::<u64>() {
      Ok(chunk_size) => chunk_size,
      Err(error) => return println!("Error: {}", error),
    };
    
    let start_index = match start_index_str.parse::<u64>() {
      Ok(start_index) => start_index,
      Err(error) => return println!("Error: {}", error),
    };
    
    start_params = Some(WipeStartingParams {
      params: WipeParams {
        drive,
        chunk_size,
      },
      start_chunk_index: start_index,
    });
  } else {
    println!("Usage:");
    println!("  New wipe:");
    println!("    drive-wiper <drive> <chunk size> <start index> <status storage file>");
    println!("  Resuming from previous wipe:");
    println!("    drive-wiper <status storage file>");
    println!("    drive-wiper <drive> <status storage file>");
    return;
  }
  
  match wipe_drive(start_params, status_storage_file_path.as_str()) {
    Ok(()) => println!("Drive wipe successful."),
    Err(err_box) => println!("Error: {}", err_box),
  }
}
