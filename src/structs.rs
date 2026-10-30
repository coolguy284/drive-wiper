use std::error::Error;
use std::fmt::Display;
use std::io::SeekFrom;

use crate::errors::WipeStatusConvertError;
use crate::io::get_drive_size;

#[derive(Debug, Clone)]
pub(crate) enum FileOffset {
  Start(u64),
  End(u64),
}

impl FileOffset {
  pub(crate) fn to_start_offset(&self, file_len: u64) -> u64 {
    match self {
      FileOffset::Start(offset) => offset.to_owned(),
      FileOffset::End(offset) => file_len - offset,
    }
  }
}

impl Display for FileOffset {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FileOffset::Start(offset) => write!(f, "start + {}", offset),
      FileOffset::End(offset) => write!(f, "end - {}", offset),
    }
  }
}

impl TryFrom<FileOffset> for SeekFrom {
  type Error = <i64 as TryInto<u64>>::Error;
  
  fn try_from(value: FileOffset) -> Result<Self, Self::Error> {
    match value {
      FileOffset::Start(offset) => Ok(SeekFrom::Start(offset)),
      FileOffset::End(offset) => {
        Ok(SeekFrom::End(offset.try_into()?))
      },
    }
  }
}

#[derive(Debug, Clone)]
pub(crate) enum WipeStartingParams {
  ResumeStatusFile {
    status_storage_file_path: String,
  },
  ResumeStatusFileWithDrive {
    status_storage_file_path: String,
    drive: String,
  },
  InitialSetup {
    status_storage_file_path: String,
    drive: String,
    chunk_size: u64,
    start_offset: FileOffset,
    end_offset: FileOffset,
  },
}

pub(crate) struct WipeStatus {
  pub(crate) drive: String,
  pub(crate) chunk_size: u64,
  pub(crate) start_offset: FileOffset,
  pub(crate) end_offset: FileOffset,
  pub(crate) drive_size: u64,
  pub(crate) current_offset: u64,
  pub(crate) finished: bool,
}

impl WipeStartingParams {
  pub(crate) fn to_wipe_status(&self) -> Result<WipeStatus, Box<dyn Error>> {
    match self {
      WipeStartingParams::ResumeStatusFile { .. } => Err(Box::new(WipeStatusConvertError)),
      
      WipeStartingParams::ResumeStatusFileWithDrive { .. } => Err(Box::new(WipeStatusConvertError)),
      
      WipeStartingParams::InitialSetup {
        drive,
        chunk_size,
        start_offset,
        end_offset,
        ..
      } => {
        let drive_size = get_drive_size(drive)?;
        
        Ok(WipeStatus {
          drive: drive.to_owned(),
          chunk_size: chunk_size.to_owned(),
          start_offset: start_offset.to_owned(),
          end_offset: end_offset.to_owned(),
          drive_size,
          current_offset: start_offset.to_start_offset(drive_size),
          finished: false,
        })
      },
    }
  }
}

pub(crate) struct FileByteIterator {
  
}
