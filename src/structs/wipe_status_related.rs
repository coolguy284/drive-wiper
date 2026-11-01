#[derive(Debug, Clone)]
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
  pub(crate) drive_size: u64,
  pub(crate) current_offset: u64,
  pub(crate) finished: bool,
}

impl WipeStartingParams {
  pub(crate) fn to_wipe_status(&self) -> Result<WipeStatus, Box<dyn Error>> {
    match self {
      WipeStartingParams::ResumeStatusFile { .. } => Err(Box::new(WipeStatusConvertError)),
      
      WipeStartingParams::ResumeStatusFileWithDrive { .. } => Err(Box::new(WipeStatusConvertError)),
      
      WipeStartingParams::InitialSetup {
        drive,
        chunk_size,
        start_offset,
        end_offset,
        ..
      } => {
        let drive_size = DriveFile::get_drive_size(drive)?;
        
        Ok(WipeStatus {
          drive: drive.to_owned(),
          chunk_size: chunk_size.to_owned(),
          start_offset: start_offset.to_owned(),
          end_offset: end_offset.to_owned(),
          drive_size,
          current_offset: start_offset.to_start_offset(drive_size),
          finished: false,
        })
      },
    }
  }
}

// start is inclusive, end is exclusive
pub(crate) struct FileByteIterator {
  chunk_size: u64,
  end_offset: u64,
  current_offset: u64,
  currently_chunk_aligned: bool,
}

impl WipeStatus {
  pub(crate) fn get_iterator(&self) -> FileByteIterator {
    FileByteIterator {
      chunk_size: self.chunk_size,
      end_offset: self.end_offset.to_start_offset(self.drive_size),
      current_offset: self.current_offset,
      currently_chunk_aligned: self.current_offset % self.chunk_size != 0,
    }
  }
}

impl Iterator for FileByteIterator {
  type Item = FileByteIteratorOutput;
  
  fn next(&mut self) -> Option<Self::Item> {
    if self.current_offset < self.end_offset {
      let (end_offset_attempt, length_attempt) = if !self.currently_chunk_aligned {
        self.currently_chunk_aligned = true;
        
        let bytes_needed_to_align = self.chunk_size - self.current_offset % self.chunk_size;
        
        (self.current_offset + bytes_needed_to_align, bytes_needed_to_align)
      } else {
        (self.current_offset + self.chunk_size, self.chunk_size)
      };
      
      if end_offset_attempt > self.end_offset {
        let result = Some(FileByteIteratorOutput {
          start_offset: self.current_offset,
          end_offset: self.end_offset,
          length: self.end_offset - self.current_offset,
        });
        
        self.current_offset = self.end_offset;
        
        result
      } else {
        let result = Some(FileByteIteratorOutput {
          start_offset: self.current_offset,
          end_offset: end_offset_attempt,
          length: length_attempt,
        });
        
        self.current_offset = end_offset_attempt;
        
        result
      }
    } else {
      None
    }
  }
}

// bound is [start, end)
pub(crate) struct FileByteIteratorOutput {
  pub(crate) start_offset: u64,
  pub(crate) end_offset: u64,
  pub(crate) length: u64,
}
