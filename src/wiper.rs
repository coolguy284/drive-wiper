use std::error::Error;
use std::io::SeekFrom;
use std::time::{Duration, Instant};

use chrono::{Local, TimeDelta};
use cli_prompts::DisplayPrompt;
use cli_prompts::prompts::Confirmation;

use crate::consts::AVERAGE_SPEED_SAMPLE_COUNT;
use crate::errors::{DriveIncorrectSizeError, ParamsNotSpecifiedError, PromptAbortedError, StartOffsetLoweredError, WipeCancelledError};
use crate::io::{read_status, write_status};
use crate::structs::drive_file::DriveFile;
use crate::structs::sum_tracker::SumTracker;
use crate::structs::wipe_status_related::WipeStartingParams;

fn is_zero(data: &[u8]) -> bool {
  // https://stackoverflow.com/questions/65367552/how-to-efficiently-check-a-vecu8-to-see-if-its-all-zeros/65376133#65376133
  
  let (prefix, chunks, suffix) = unsafe { data.align_to::<u128>() };
  
  for byte in prefix {
    if byte.to_owned() != 0 {
      return false
    }
  }
  
  for chunk in chunks {
    if chunk.to_owned() != 0 {
      return false
    }
  }
  
  for byte in suffix {
    if byte.to_owned() != 0 {
      return false
    }
  }
  
  true
}

fn format_time(time: f64) -> String {
  if time < 0.0 {
    format!("-{}", format_time(-time))
  } else if time < 60.0 {
    format!("{:.1}s", time)
  } else if time < 60.0 * 60.0 {
    format!("{:.0}m {:04.1}s", f64::floor(time / 60.0), time % 60.0)
  } else if time < 24.0 * 60.0 * 60.0 {
    format!("{:.0}h {:02.0}m {:04.1}s", f64::floor(time / (60.0 * 60.0)), f64::floor(time / 60.0 % 60.0), time % 60.0)
  } else {
    format!("{:.0}d {:02.0}h {:02.0}m {:04.1}s", f64::floor(time / (24.0 * 60.0 * 60.0)), f64::floor(time / (60.0 * 60.0) % 24.0), f64::floor(time / 60.0 % 60.0), time % 60.0)
  }
}

pub(crate) fn wipe_drive(start_params: WipeStartingParams) -> Result<(), Box<dyn Error>> {
  let status_storage_file_path = match start_params.clone() {
    WipeStartingParams::ResumeStatusFile {
      status_storage_file_path 
    } => status_storage_file_path,
    
    WipeStartingParams::ResumeStatusFileWithDrive {
      status_storage_file_path,
      ..
    } => status_storage_file_path,
    
    WipeStartingParams::InitialSetup {
      status_storage_file_path,
      ..
    } => status_storage_file_path,
  };
  
  let mut status = match read_status(status_storage_file_path.as_str())? {
    Some(mut status) => match start_params {
      WipeStartingParams::ResumeStatusFile {
        ..
      } => status,
      
      WipeStartingParams::ResumeStatusFileWithDrive {
        drive,
        ..
      } => {
        status.drive = drive;
        
        status
      },
      
      WipeStartingParams::InitialSetup {
        drive,
        chunk_size,
        start_offset,
        end_offset,
        drive_write_sync,
        ..
      } => {
        let original_start_offset_from_start = status.start_offset.to_start_offset(status.drive_size);
        let new_start_offset_from_start = start_offset.to_start_offset(status.drive_size);
        
        if original_start_offset_from_start < new_start_offset_from_start {
          return Err(Box::new(StartOffsetLoweredError {
            original_offset: original_start_offset_from_start,
            new_offset: new_start_offset_from_start,
          }))
        }
        
        status.drive = drive;
        status.chunk_size = chunk_size;
        status.start_offset = start_offset;
        status.end_offset = end_offset;
        status.drive_write_sync = drive_write_sync;
        
        status
      },
    },
    
    None => match start_params {
      WipeStartingParams::ResumeStatusFile {
        ..
      } => return Err(Box::new(ParamsNotSpecifiedError)),
      
      WipeStartingParams::ResumeStatusFileWithDrive {
        ..
      } => return Err(Box::new(ParamsNotSpecifiedError)),
      
      WipeStartingParams::InitialSetup {
        ..
      } => start_params.to_wipe_status()?,
    },
  };
  
  if status.finished {
    println!("Drive wipe file is for an already finished drive wipe");
    println!("If this was not intentional, delete the status file and try again");
    
    return Ok(());
  }
  
  println!("Drive: {}", status.drive);
  println!("Chunk Size: {} B", status.chunk_size);
  println!("Start Offset: {} B", status.start_offset);
  println!("End Offset: {} B", status.end_offset);
  println!("Use Sync Writes for Drive: {}", if status.drive_write_sync { "yes" } else { "no" });
  println!("Drive Size: {} B", status.drive_size);
  println!("Current Offset: {} B", status.current_offset);
  println!("Finished: {}", status.finished);
  println!();
  
  let answer =
    Confirmation::new("Are you sure you want to wipe the drive?")
      .default_positive(false)
      .display();
  
  match answer {
    Ok(is_positive) => {
      if !is_positive {
        return Err(Box::new(WipeCancelledError));
      }
    },
    Err(abort_reason) => return Err(Box::new(PromptAbortedError { abort_reason })),
  }
  
  println!("Confirmation acquired, proceeding with wipe...");
  println!();
  
  let mut file = DriveFile::open(status.drive.as_str())?;
  
  let drive_length = file.size()?;
  
  if drive_length != status.drive_size {
    return Err(Box::new(DriveIncorrectSizeError {
      expected_size: status.drive_size,
      actual_size: drive_length,
    }))
  }
  
  let start_offset = status.start_offset.to_start_offset(drive_length);
  let end_offset = status.end_offset.to_start_offset(drive_length) - 1;
  
  let end_offset_string = format!("{}", end_offset);
  
  let mut past_instant;
  let mut current_instant = None;
  let mut duration_sum: SumTracker<Duration, AVERAGE_SPEED_SAMPLE_COUNT> = SumTracker::new();
  let mut bytes_sum: SumTracker<u64, AVERAGE_SPEED_SAMPLE_COUNT> = SumTracker::new();
  
  for iteration in status.get_iterator() {
    past_instant = current_instant;
    current_instant = Some(Instant::now());
    
    let rate_string = if let Some(past_instant_value) = past_instant {
      if let Some(current_instant_value) = current_instant {
        let elapsed_time = current_instant_value - past_instant_value;
        
        duration_sum.add_data(elapsed_time);
        bytes_sum.add_data(iteration.length);
        
        let data_rate = (bytes_sum.get_sum() as f64) / duration_sum.get_sum().as_secs_f64();
        let time_left = (((end_offset + 1) - iteration.start_offset) as f64) / data_rate;
        let eta = Local::now() + TimeDelta::milliseconds((time_left * 1000.0) as i64);
        
        format!(
          ", {:.0} B/s, ETA {}{} {}, {} left",
          data_rate,
          eta.format("%F %T."),
          eta.timestamp_subsec_millis() / 100,
          eta.format("%:z"),
          format_time(time_left),
        )
      } else {
        "".to_string()
      }
    } else {
      "".to_string()
    };
    
    let progress = ((iteration.start_offset - start_offset) as f64) / (((end_offset + 1) - start_offset) as f64);
    
    print!(
      "Checking {1:00$}-{2:00$}/{3} B ({4:09.6}%){5}: ",
      end_offset_string.len(),
      iteration.start_offset,
      iteration.end_offset - 1,
      end_offset_string,
      progress * 100.0,
      rate_string,
    );
    
    let mut data: Vec<u8> = vec![];
    
    data.resize(iteration.length.try_into()?, 0);
    
    file.seek(SeekFrom::Start(iteration.start_offset))?;
    file.read_exact(data.as_mut_slice())?;
    
    if !is_zero(data.as_slice()) {
      println!("Found nonzero data, erasing...");
      
      data.clear();
      data.resize(iteration.length.try_into()?, 0);
      
      file.seek(SeekFrom::Start(iteration.start_offset))?;
      file.write_all(data.as_slice())?;
      
      if status.drive_write_sync {
        file.sync_data()?;
      }
    } else {
      println!("Already zero");
    }
    
    status.current_offset = iteration.end_offset;
    
    write_status(&status, status_storage_file_path.as_str())?;
  }
  
  status.finished = true;
  
  write_status(&status, status_storage_file_path.as_str())?;
  
  Ok(())
}
