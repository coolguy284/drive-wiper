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
