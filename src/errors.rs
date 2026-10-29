#[derive(Debug, Clone)]
pub(crate) struct StatusFormatError;

impl fmt::Display for StatusFormatError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Invalid format for status file")?;
    Ok(())
  }
}

impl Error for StatusFormatError {}

#[derive(Debug, Clone)]
pub(crate) struct StatusVersionError {
  expected_ver: u64,
  actual_ver: u64,
}

impl fmt::Display for StatusVersionError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Status version incorrect: expected {}, found {}", self.expected_ver, self.actual_ver)?;
    Ok(())
  }
}

impl Error for StatusVersionError {}

#[derive(Debug, Clone)]
pub(crate) struct ArgDecodeError {
  arg_index: usize,
}

impl fmt::Display for ArgDecodeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Unable to decode argument with index {}", self.arg_index)?;
    Ok(())
  }
}

impl Error for ArgDecodeError {}

#[derive(Debug, Clone)]
pub(crate) struct ParamsNotSpecifiedError;

impl fmt::Display for ParamsNotSpecifiedError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Status file not present but drive wiping parameters not specified")?;
    Ok(())
  }
}

impl Error for ParamsNotSpecifiedError {}
