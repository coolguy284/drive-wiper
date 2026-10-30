use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use cli_prompts::DisplayPrompt;
use cli_prompts::prompts::Confirmation;

use crate::errors::{DriveIncorrectSizeError, ParamsNotSpecifiedError, PromptAbortedError, WipeCancelledError};
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
      //.style(ConfirmationStyle::default()) (TODO: remove this if this line ends up not being needed)
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
  
  let drive_length = file.metadata()?.len();
  
  if drive_length != status.drive_size {
    return Err(Box::new(DriveIncorrectSizeError {
      expected_size: status.drive_size,
      actual_size: drive_length,
    }))
  }
  
  let start_offset_from_start = status.start_offset.to_start_offset(drive_length);
  let end_offset_from_start = status.end_offset.to_start_offset(drive_length);
  
  for offset in (start_offset_from_start..=end_offset_from_start).step_by(status.chunk_size) {
    
  }
  
  file.seek(SeekFrom::Start(status.current_offset))?;
  file.read_exact(buf);
  // TODO
  // do fsync to be really sure
  
  Ok(())
}
