use std::error::Error;
use std::fs::File;

use crate::errors::ParamsNotSpecifiedError;
use crate::io::read_prior_status;
use crate::structs::{WipeStartingParams, WipeStatus};

pub(crate) fn wipe_drive(start_params_opt: Option<WipeStartingParams>, status_storage_file_path: &str) -> Result<(), Box<dyn Error>> {
  let mut status = match read_prior_status(status_storage_file_path)? {
    Some(status) => status,
    None => match start_params_opt {
      Some(start_params) => WipeStatus::from(start_params.params),
      None => return Err(Box::new(ParamsNotSpecifiedError)),
    },
  };
  
  File::options()
    .read(true)
    .write(true)
    .open(status_storage_file_path)?;
  
  // TODO
  
  Ok(())
}
