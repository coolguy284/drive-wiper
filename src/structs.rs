pub(crate) struct WipeParams {
  pub(crate) drive: String,
  pub(crate) chunk_size: u64,
}

pub(crate) enum FileOffset {
  Start(u64),
  End(u64),
}

pub(crate) enum WipeStartingParams {
  ResumeStatusFile {
    status_storage_file_path: String,
  },
  ResumeStatusFileWithDrive {
    drive: String,
    status_storage_file_path: String,
  },
  InitialSetup {
    drive: String,
    status_storage_file_path: String,
    chunk_size: u64,
    start_offset: FileOffset,
    end_offset: FileOffset,
  },
}

pub(crate) struct WipeStatus {
  pub(crate) params: WipeParams,
  pub(crate) chunk_index: u64,
  pub(crate) finished: bool,
}

impl From<WipeParams> for WipeStatus {
  fn from(value: WipeParams) -> Self {
    WipeStatus {
      params: value,
      chunk_index: 0,
      finished: false,
    }
  }
}
