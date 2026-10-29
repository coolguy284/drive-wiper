use std::env::ArgsOs;

use crate::errors::ArgDecodeError;

pub(crate) fn decode_prgm_args(prgm_args: ArgsOs) -> Result<Vec<String>, ArgDecodeError> {
  // https://doc.rust-lang.org/std/result/enum.Result.html#method.from_iter
  
  let converted_args: Result<Vec<String>, ArgDecodeError> =
    prgm_args
      .enumerate()
      .map(|(index, os_str)| match os_str.into_string() {
        Ok(str) => Ok(str),
        Err(_) => Err(ArgDecodeError { arg_index: index }),
      })
      .collect();
  
  return converted_args;
}
