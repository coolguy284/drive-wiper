use std::env::args_os;
use std::error::Error;

mod args;
mod consts;
mod errors;
mod io;
mod structs;

fn wipe_drive(start_params_opt: Option<WipeStartingParams>, status_storage_file_path: String) -> Result<(), Box<dyn Error>> {
  let mut status = match read_prior_status(status_storage_file_path)? {
    Some(status) => status,
    None => match start_params_opt {
      Some(start_params) => WipeStatus::from(start_params.params),
      None => return Err(Box::new(ParamsNotSpecifiedError)),
    },
  };
  
  todo!();
  
  Ok(())
}

fn main() {
  let prgm_args = match decode_prgm_args(args_os()) {
    Ok(prgm_args) => prgm_args,
    Err(error) => {
      println!("Error: {}", error);
      return;
    },
  };
  
  if prgm_args.len() == 1 {
    let status_storage_file_path = prgm_args[1].clone();
    
    match wipe_drive(None, status_storage_file_path) {
      Ok(()) => println!("Drive wipe successful."),
      Err(err_box) => println!("Error: {}", err_box),
    }
  } else if prgm_args.len() == 3 {
    let drive = prgm_args[1].clone();
    let chunk_size_str = prgm_args[2].clone();
    let start_index_str = prgm_args[3].clone();
    let status_storage_file_path = prgm_args[4].clone();
    
    let chunk_size = match chunk_size_str.parse::<u64>() {
      Ok(chunk_size) => chunk_size,
      Err(error) => return println!("Error: {}", error),
    };
    
    let start_index = match start_index_str.parse::<u64>() {
      Ok(start_index) => start_index,
      Err(error) => return println!("Error: {}", error),
    };
    
    match
      wipe_drive(
        Some(WipeStartingParams {
          params: WipeParams {
            drive,
            chunk_size,
          },
          start_chunk_index: start_index,
        }),
        status_storage_file_path,
      )
    {
      Ok(()) => println!("Drive wipe successful."),
      Err(err_box) => println!("Error: {}", err_box),
    }
  } else {
    println!("Usage:");
    println!("  drive-wiper <drive> <chunk size> <start index> <status storage file>");
    println!("  drive-wiper <status storage file>");
  }
}
