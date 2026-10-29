pub(crate) struct WipeParams {
  drive: String,
  chunk_size: u64,
}

pub(crate) struct WipeStartingParams {
  params: WipeParams,
  start_chunk_index: u64,
}

pub(crate) struct WipeStatus {
  params: WipeParams,
  chunk_index: u64,
  finished: bool,
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
