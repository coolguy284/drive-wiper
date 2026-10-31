use std::error::Error;
use std::fmt;

use cli_prompts::prompts::AbortReason;

#[derive(Debug, Clone)]
pub(crate) struct ArgDecodeError {
  pub(crate) arg_index: usize,
}

impl fmt::Display for ArgDecodeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Unable to decode argument with index {}", self.arg_index)?;
    Ok(())
  }
}

impl Error for ArgDecodeError {}

#[derive(Debug, Clone)]
pub(crate) struct ArgFormatError {
  pub(crate) arg_index: usize,
}

impl fmt::Display for ArgFormatError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Bad format for argument with index {}", self.arg_index)?;
    Ok(())
  }
}

impl Error for ArgFormatError {}

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
  pub(crate) expected_ver: u64,
  pub(crate) actual_ver: u64,
}

impl fmt::Display for StatusVersionError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Status version incorrect: expected {}, found {}", self.expected_ver, self.actual_ver)?;
    Ok(())
  }
}

impl Error for StatusVersionError {}

#[derive(Debug, Clone)]
pub(crate) struct ParamsNotSpecifiedError;

impl fmt::Display for ParamsNotSpecifiedError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Status file not present but drive wiping parameters not specified")?;
    Ok(())
  }
}

impl Error for ParamsNotSpecifiedError {}


#[derive(Debug, Clone)]
pub(crate) struct WipeStatusConvertError;

impl fmt::Display for WipeStatusConvertError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Expected enum variant WipeStartingParams::InitialSetup to convert to WipeStatus")?;
    Ok(())
  }
}

impl Error for WipeStatusConvertError {}

#[derive(Debug, Clone)]
pub(crate) struct WipeCancelledError;

impl fmt::Display for WipeCancelledError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Drive wipe cancelled")?;
    Ok(())
  }
}

impl Error for WipeCancelledError {}

#[derive(Debug)]
pub(crate) struct PromptAbortedError {
  pub(crate) abort_reason: AbortReason,
}

impl fmt::Display for PromptAbortedError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Prompt aborted with reason: {:?}", self.abort_reason)?;
    Ok(())
  }
}

impl Error for PromptAbortedError {}

#[derive(Debug)]
pub(crate) struct DriveIncorrectSizeError {
  pub(crate) expected_size: u64,
  pub(crate) actual_size: u64,
}

impl fmt::Display for DriveIncorrectSizeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Unexpected size for drive: expected {}, actual {}", self.expected_size, self.actual_size)?;
    Ok(())
  }
}

impl Error for DriveIncorrectSizeError {}

#[derive(Debug)]
pub(crate) struct StartOffsetLoweredError {
  pub(crate) original_offset: u64,
  pub(crate) new_offset: u64,
}

impl fmt::Display for StartOffsetLoweredError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Wipe start offset was lowered: originally {}, now {}", self.original_offset, self.new_offset)?;
    Ok(())
  }
}

impl Error for StartOffsetLoweredError {}
