// identical signature to std::fs::write for parity
pub(crate) fn write_atomic<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> std::io::Result<()> {
  let mut file = AtomicWriteFile::open(path)?;
  file.write_all(contents.as_ref())?;
  Ok(())
}

pub(crate) fn read_prior_status(status_storage_file_path: String) -> Result<Option<WipeStatus>, Box<dyn Error>> {
  // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
  let data_binary: Vec<u8> = match read(status_storage_file_path) {
    Ok(data) => data,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => return Ok(None),
      _ => return Err(Box::new(error)),
    },
  };
  
  let data_string = match from_utf8(&data_binary) {
    Ok(data) => data,
    Err(error) => return Err(Box::new(error)),
  };
  
  let data_json = match parse(data_string) {
    Ok(data) => data,
    Err(error) => return Err(Box::new(error)),
  };
  
  if !matches!(data_json, JsonValue::Object(_)) {
    return Err(Box::new(StatusFormatError))
  }
  
  let Some(version) = data_json["version"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  
  if version != STATUS_VERSION {
    return Err(Box::new(StatusVersionError {
      expected_ver: STATUS_VERSION,
      actual_ver: version,
    }));
  }
  
  let params = &data_json["params"];
  
  if !matches!(params, JsonValue::Object(_)) {
    return Err(Box::new(StatusFormatError))
  }
  
  let Some(drive_str) = params["drive"].as_str() else { return Err(Box::new(StatusFormatError)) };
  let Some(chunk_size) = params["chunk_size"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  let Some(chunk_index) = data_json["chunk_index"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  let Some(finished) = data_json["finished"].as_bool() else { return Err(Box::new(StatusFormatError)) };
  
  Ok(Some(WipeStatus {
    params: WipeParams {
      drive: drive_str.to_owned(),
      chunk_size,
    },
    chunk_index,
    finished,
  }))
}

pub(crate) fn write_prior_status(status: WipeStatus, status_storage_file_path: String) -> Result<(), Box<dyn Error>> {
  let data_json = object!{
    version: STATUS_VERSION,
    params: {
      drive: status.params.drive,
      chunk_size: status.params.chunk_size,
    },
    chunk_index: status.chunk_index,
    finished: status.finished,
  };
  
  let data_string = stringify_pretty(data_json, STATUS_JSON_INDENT);
  
  let data_binary = data_string.as_bytes();
  
  match write_atomic(status_storage_file_path, data_binary) {
    Ok(()) => Ok(()),
    Err(error) => return Err(Box::new(error)),
  }
}
