use std::env::args_os;
use std::error::Error;

use crate::args::decode_prgm_args;
use crate::errors::ArgFormatError;
use crate::structs::file_offset::FileOffset;
use crate::structs::wipe_status_related::WipeStartingParams;
use crate::wiper::wipe_drive;

#[cfg(target_os = "windows")]
use std::sync::LazyLock;

#[cfg(target_os = "windows")]
use crate::consts::{DRIVE_PARTITION_WIN_REGEX, PHYSICAL_DRIVE_WIN_REGEX};

mod consts;
mod errors;
mod structs;
mod traits;
mod args;
mod io;
mod wiper;

fn main() -> Result<(), Box<dyn Error>> {
  // Get the panics out of the way at the start of the program in debug mode
  #[cfg(target_os = "windows")]
  #[cfg(debug_assertions)]
  {
    LazyLock::force(&PHYSICAL_DRIVE_WIN_REGEX);
    LazyLock::force(&DRIVE_PARTITION_WIN_REGEX);
  }
  
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
  } else if prgm_args.len() == 9 {
    let chunk_size = prgm_args[2].clone().parse::<u64>()?;
    let start_offset_basis = prgm_args[3].clone();
    let start_offset = prgm_args[4].clone().parse::<u64>()?;
    let end_offset_basis = prgm_args[5].clone();
    let end_offset = prgm_args[6].clone().parse::<u64>()?;
    
    start_params = WipeStartingParams::InitialSetup {
      status_storage_file_path: prgm_args[7].clone(),
      drive: prgm_args[1].clone(),
      chunk_size,
      start_offset: match start_offset_basis.as_str() {
        "start" => FileOffset::Start(start_offset),
        "end" => FileOffset::End(start_offset),
        _ => return Err(Box::new(ArgFormatError { arg_index: 3 })),
      },
      end_offset: match end_offset_basis.as_str() {
        "start" => FileOffset::Start(end_offset),
        "end" => FileOffset::End(end_offset),
        _ => return Err(Box::new(ArgFormatError{ arg_index: 5 })),
      },
      drive_write_sync: match prgm_args[8].as_str() {
        "yes" => true,
        "no" => false,
        _ => return Err(Box::new(ArgFormatError{ arg_index: 8 })),
      },
    };
  } else {
    println!("Usage:");
    println!("  New wipe:");
    println!("    drive-wiper <drive> <chunk size> <start offset basis> <start offset> <end offset basis> <end offset> <status storage file> <sync writes to drive>");
    println!("    (Both end and start index are inclusive)");
    println!("  Resuming from previous wipe:");
    println!("    drive-wiper <status storage file>");
    println!("    drive-wiper <drive> <status storage file>");
    return Ok(());
  }
  
  wipe_drive(start_params)?;
  
  println!("Drive wipe successful.");
  
  Ok(())
}
