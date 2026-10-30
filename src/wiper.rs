use std::error::Error;
use std::fs::File;
use std::io::{Seek, SeekFrom};

use cli_prompts::DisplayPrompt;
use cli_prompts::prompts::Confirmation;

use crate::errors::{ParamsNotSpecifiedError, PromptAbortedError, WipeCancelledError};
use crate::io::read_prior_status;
use crate::structs::WipeStartingParams;

pub(crate) fn wipe_drive(start_params: WipeStartingParams) -> Result<(), Box<dyn Error>> {
  let status_storage_file_path = match start_params.clone() {
    WipeStartingParams::ResumeStatusFile {
      status_storage_file_path 
    } => status_storage_file_path,
    
    WipeStartingParams::ResumeStatusFileWithDrive {
      status_storage_file_path,
      ..
    } => status_storage_file_path,
    
    WipeStartingParams::InitialSetup {
      status_storage_file_path,
      ..
    } => status_storage_file_path,
  };
  
  let mut status = match read_prior_status(status_storage_file_path.as_str())? {
    Some(status) => status,
    None => match start_params {
      WipeStartingParams::ResumeStatusFile {
        ..
      } => return Err(Box::new(ParamsNotSpecifiedError)),
      
      WipeStartingParams::ResumeStatusFileWithDrive {
        ..
      } => return Err(Box::new(ParamsNotSpecifiedError)),
      
      WipeStartingParams::InitialSetup {
        ..
      } => start_params.to_wipe_status()?,
    },
  };
  
  println!("Drive: {}", status.drive);
  println!("Chunk Size: {} B", status.chunk_size);
  println!("Start Offset: {} B", status.start_offset);
  println!("End Offset: {} B", status.end_offset);
  println!("Drive Size: {} B", status.drive_size);
  println!("Current Offset: {} B", status.current_offset);
  println!("Finished: {}", status.finished);
  println!();
  
  let answer =
    Confirmation::new("Are you sure you want to wipe the drive?")
      .default_positive(false)
      .display();
  
  match answer {
    Ok(is_positive) => {
      if !is_positive {
        return Err(Box::new(WipeCancelledError));
      }
    }
    Err(abort_reason) => return Err(Box::new(PromptAbortedError { abort_reason })),
  }
  
  println!("Confirmation acquired, proceeding with wipe...");
  println!();
  
  let mut file =
    File::options()
      .read(true)
      .write(true)
      .open(status_storage_file_path)?;
  
  file.seek(SeekFrom::Start(status.current_offset))?;
  // TODO
  // do fsync to be really sure
  
  Ok(())
}
