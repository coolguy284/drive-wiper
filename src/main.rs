use std::env::args_os;
use std::error::Error;

use crate::args::decode_prgm_args;
use crate::structs::WipeStartingParams;
use crate::wiper::wipe_drive;

mod args;
mod consts;
mod errors;
mod io;
mod structs;
mod wiper;

fn main() -> Result<(), Box<dyn Error>> {
  let prgm_args = decode_prgm_args(args_os())?;
  
  let start_params;
  
  if prgm_args.len() == 2 {
    start_params = WipeStartingParams::ResumeStatusFile {
      status_storage_file_path: prgm_args[1].clone(),
    };
  } else if prgm_args.len() == 3 {
    start_params = WipeStartingParams::ResumeStatusFileWithDrive {
      status_storage_file_path: prgm_args[2].clone(),
      drive: prgm_args[1].clone(),
    };
  } else if prgm_args.len() == 6 {
    let chunk_size = prgm_args[2].clone().parse::<u64>()?;
    let start_offset = prgm_args[3].clone().parse::<u64>()?;
    let end_offset = prgm_args[3].clone().parse::<u64>()?;
    
    start_params = WipeStartingParams::InitialSetup {
      status_storage_file_path: prgm_args[4].clone(),
      drive: prgm_args[1].clone(),
      chunk_size,
      start_offset,
      end_offset,
    };
  } else {
    println!("Usage:");
    println!("  New wipe:");
    println!("    drive-wiper <drive> <chunk size> <start offset> <end offset> <status storage file>");
    println!("  Resuming from previous wipe:");
    println!("    drive-wiper <status storage file>");
    println!("    drive-wiper <drive> <status storage file>");
    return Ok(());
  }
  
  wipe_drive(start_params)?;
  
  println!("Drive wipe successful.");
  
  Ok(())
}
