use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use cli_prompts::DisplayPrompt;
use cli_prompts::prompts::Confirmation;

use crate::errors::{DriveIncorrectSizeError, ParamsNotSpecifiedError, PromptAbortedError, StartOffsetLoweredError, WipeCancelledError};
use crate::io::{read_status, write_status};
use crate::structs::WipeStartingParams;

fn is_zero(data: &[u8]) -> bool {
  // https://stackoverflow.com/questions/65367552/how-to-efficiently-check-a-vecu8-to-see-if-its-all-zeros/65376133#65376133
  
  let (prefix, chunks, suffix) = unsafe { data.align_to::<u128>() };
  
  for byte in prefix {
    if byte.to_owned() != 0 {
      return false
    }
  }
  
  for chunk in chunks {
    if chunk.to_owned() != 0 {
      return false
    }
  }
  
  for byte in suffix {
    if byte.to_owned() != 0 {
      return false
    }
  }
  
  true
}

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
  
  let mut status = match read_status(status_storage_file_path.as_str())? {
    Some(mut status) => match start_params {
      WipeStartingParams::ResumeStatusFile {
        ..
      } => status,
      
      WipeStartingParams::ResumeStatusFileWithDrive {
        drive,
        ..
      } => {
        status.drive = drive;
        
        status
      },
      
      WipeStartingParams::InitialSetup {
        drive,
        chunk_size,
        start_offset,
        end_offset,
        ..
      } => {
        let original_start_offset_from_start = status.start_offset.to_start_offset(status.drive_size);
        let new_start_offset_from_start = start_offset.to_start_offset(status.drive_size);
        
        if original_start_offset_from_start < new_start_offset_from_start {
          return Err(Box::new(StartOffsetLoweredError {
            original_offset: original_start_offset_from_start,
            new_offset: new_start_offset_from_start,
          }))
        }
        
        status.drive = drive;
        status.chunk_size = chunk_size;
        status.start_offset = start_offset;
        status.end_offset = end_offset;
        
        status
      },
    },
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
  
  if status.finished {
    println!("Drive wipe file is for an already finished drive wipe");
    println!("If this was not intentional, delete the status file and try again");
    
    return Ok(());
  }
  
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
      .open(status_storage_file_path.as_str())?;
  
  let drive_length = file.metadata()?.len();
  
  if drive_length != status.drive_size {
    return Err(Box::new(DriveIncorrectSizeError {
      expected_size: status.drive_size,
      actual_size: drive_length,
    }))
  }
  
  for iteration in status.get_iterator() {
    println!("Erasing bytes {}-{}/{}", iteration.start_offset, iteration.end_offset - 1, drive_length);
    
    let mut data: Vec<u8> = vec![];
    
    data.resize(iteration.length.try_into()?, 0);
    
    file.seek(SeekFrom::Start(iteration.start_offset))?;
    file.read_exact(data.as_mut_slice())?;
    
    if !is_zero(data.as_slice()) {
      println!("Found nonzero data, erasing...");
      
      file.seek(SeekFrom::Start(iteration.start_offset))?;
      file.write_all(data.as_slice())?;
      file.sync_data()?;
    }
    
    status.current_offset = iteration.end_offset;
    
    write_status(&status, status_storage_file_path.as_str())?;
  }
  
  status.finished = true;
  
  write_status(&status, status_storage_file_path.as_str())?;
  
  Ok(())
}
