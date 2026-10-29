pub(crate) struct WipeParams {
  pub(crate) drive: String,
  pub(crate) chunk_size: u64,
}

pub(crate) struct WipeStartingParams {
  pub(crate) params: WipeParams,
  pub(crate) start_chunk_index: u64,
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
