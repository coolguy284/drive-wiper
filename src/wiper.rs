use std::error::Error;
use std::fs::File;
use std::io::{Seek, SeekFrom};

use crate::errors::ParamsNotSpecifiedError;
use crate::io::read_prior_status;
use crate::structs::{WipeStartingParams, WipeStatus};

pub(crate) fn wipe_drive(start_params_opt: WipeStartingParams) -> Result<(), Box<dyn Error>> {
  let status_storage_file_path
  
  let mut status = match read_prior_status(start_params_opt.status_storage_file_path)? {
    Some(status) => status,
    None => match start_params_opt {
      Some(start_params) => WipeStatus::from(start_params.params),
      None => return Err(Box::new(ParamsNotSpecifiedError)),
    },
  };
  
  let mut file =
    File::options()
      .read(true)
      .write(true)
      .open(status_storage_file_path)?;
  
  file.seek(SeekFrom::Start(status.chunk_index * status.params.chunk_size))?;
  // TODO
  
  Ok(())
}
