use crate::errors::WipeStatusConvertError;

pub(crate) enum FileOffset {
  Start(u64),
  End(u64),
}

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
  pub(crate) current_offset: u64,
  pub(crate) finished: bool,
}

impl From<WipeStartingParams> for Result<WipeStatus, WipeStatusConvertError> {
  fn from(value: WipeStartingParams) -> Self {
    match value {
      WipeStartingParams::ResumeStatusFile { status_storage_file_path } => todo!(),
      WipeStartingParams::ResumeStatusFileWithDrive { status_storage_file_path, drive } => todo!(),
      WipeStartingParams::InitialSetup { status_storage_file_path, drive, chunk_size, start_offset, end_offset } => Ok(WipeStatus {
        drive,
        chunk_size,
        start_offset,
        end_offset,
        current_offset: todo!(),
        finished: todo!(),
      }),
    }
  }
}
