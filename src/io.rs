use std::error::Error;
use std::fs::read;
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::str::from_utf8;

use atomic_write_file::AtomicWriteFile;
use json::{JsonValue, object, parse, stringify_pretty};

use crate::consts::{STATUS_JSON_INDENT, STATUS_VERSION};
use crate::errors::{StatusFormatError, StatusVersionError};
use crate::structs::file_offset::FileOffset;
use crate::structs::wipe_status_related::{WipeStatus, WipeStatusV1};

// identical signature to std::fs::write for parity
pub(crate) fn write_atomic<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> std::io::Result<()> {
  let mut file = AtomicWriteFile::open(path)?;
  file.write_all(contents.as_ref())?;
  file.commit()?;
  Ok(())
}

fn parse_offset_json(offset: &JsonValue) -> Result<FileOffset, Box<StatusFormatError>> {
  if !matches!(offset, JsonValue::Object(_)) {
    return Err(Box::new(StatusFormatError))
  }
  
  let Some(offset_basis) = offset["basis"].as_str() else { return Err(Box::new(StatusFormatError)) };
  
  let Some(offset_value) = offset["offset"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  
  match offset_basis {
    "start" => Ok(FileOffset::Start(offset_value)),
    "end" => Ok(FileOffset::End(offset_value)),
    _ => return Err(Box::new(StatusFormatError)),
  }
}

fn offset_to_json(offset: FileOffset) -> JsonValue {
  match offset {
    FileOffset::Start(value) => object! {
      basis: "start",
      offset: value,
    },
    
    FileOffset::End(value) => object! {
      basis: "end",
      offset: value,
    },
  }
}

fn read_status_v1(data_json: JsonValue) -> Result<WipeStatusV1, Box<dyn Error>> {
  let Some(drive_str) = data_json["drive"].as_str() else { return Err(Box::new(StatusFormatError)) };
  let Some(chunk_size) = data_json["chunk_size"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  
  let start_offset = &data_json["start_offset"];
  
  let end_offset = &data_json["end_offset"];
  
  let Some(drive_size) = data_json["drive_size"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  let Some(current_offset) = data_json["current_offset"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  let Some(finished) = data_json["finished"].as_bool() else { return Err(Box::new(StatusFormatError)) };
  
  Ok(WipeStatusV1 {
    drive: drive_str.to_owned(),
    chunk_size,
    start_offset: parse_offset_json(start_offset)?,
    end_offset: parse_offset_json(end_offset)?,
    drive_size,
    current_offset,
    finished,
  })
}

fn convert_status_v1_to_v2(status: WipeStatusV1) -> WipeStatus {
  WipeStatus {
    drive: status.drive,
    chunk_size: status.chunk_size,
    start_offset: status.start_offset,
    end_offset: status.end_offset,
    drive_write_sync: true,
    drive_size: status.drive_size,
    current_offset: status.current_offset,
    finished: status.finished,
  }
}

fn read_status_v2(data_json: JsonValue) -> Result<WipeStatus, Box<dyn Error>> {
  let Some(drive_str) = data_json["drive"].as_str() else { return Err(Box::new(StatusFormatError)) };
  let Some(chunk_size) = data_json["chunk_size"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  
  let start_offset = &data_json["start_offset"];
  
  let end_offset = &data_json["end_offset"];
  
  let Some(drive_write_sync) = data_json["drive_write_sync"].as_bool() else { return Err(Box::new(StatusFormatError)) };
  
  let Some(drive_size) = data_json["drive_size"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  let Some(current_offset) = data_json["current_offset"].as_u64() else { return Err(Box::new(StatusFormatError)) };
  let Some(finished) = data_json["finished"].as_bool() else { return Err(Box::new(StatusFormatError)) };
  
  Ok(WipeStatus {
    drive: drive_str.to_owned(),
    chunk_size,
    start_offset: parse_offset_json(start_offset)?,
    end_offset: parse_offset_json(end_offset)?,
    drive_write_sync,
    drive_size,
    current_offset,
    finished,
  })
}

pub(crate) fn read_status(status_storage_file_path: &str) -> Result<Option<WipeStatus>, Box<dyn Error>> {
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
  
  match version {
    1 => convert_status_v1_to_v2(read_status_v1(status)?),
    STATUS_VERSION => read_status_v2(status)?,
    _ => Err(Box::new(StatusVersionError {
      expected_ver: STATUS_VERSION,
      actual_ver: version,
    }))
  }
}

pub(crate) fn write_status(status: &WipeStatus, status_storage_file_path: &str) -> Result<(), Box<dyn Error>> {
  let data_json = object! {
    version: STATUS_VERSION,
    drive: status.drive.clone(),
    chunk_size: status.chunk_size,
    start_offset: offset_to_json(status.start_offset.clone()),
    end_offset: offset_to_json(status.end_offset.clone()),
    drive_size: status.drive_size,
    current_offset: status.current_offset,
    finished: status.finished,
  };
  
  let data_string = stringify_pretty(data_json, STATUS_JSON_INDENT) + "\n";
  
  let data_binary = data_string.as_bytes();
  
  match write_atomic(status_storage_file_path, data_binary) {
    Ok(()) => Ok(()),
    Err(error) => return Err(Box::new(error)),
  }
}
