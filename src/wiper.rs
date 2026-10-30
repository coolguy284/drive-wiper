use std::error::Error;
use std::fs::File;
use std::io::{Seek, SeekFrom};

use crate::errors::ParamsNotSpecifiedError;
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
  
  // TODO: add confirm to begin wipe, print out info first
  
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
