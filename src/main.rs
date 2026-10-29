use std::{env::args};

fn main() {
  let prgm_args = args();
  
  if prgm_args.len() < 3 {
    println!("Usage:");
    println!("  drive-wiper <drive> <status storage file>");
  }
}
