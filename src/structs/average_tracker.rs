use std::ops::{AddAssign, SubAssign};

use num::Zero;

use crate::structs::sum_tracker::SumTracker;

// https://www.geeksforgeeks.org/rust/rust-multiple-bounds/
pub(crate) struct AverageTracker<T: Clone + AddAssign + SubAssign + Zero, const DATA_POINT_LENGTH: usize> {
  sum_tracker: SumTracker<T, DATA_POINT_LENGTH>,
  data_added: bool,
}

impl<T: Clone + AddAssign + SubAssign + Zero, const DATA_POINT_LENGTH: usize> AverageTracker<T, DATA_POINT_LENGTH> {
  pub(crate) fn new() -> Self {
    // https://stackoverflow.com/questions/76254481/how-do-i-limit-the-values-allowed-in-a-generic-const-parameter/76254798#76254798
    const { assert!(DATA_POINT_LENGTH > 0, "data_point_length must be positive") };
    
    AverageTracker {
      sum_tracker: SumTracker::new(),
      data_added: false,
    }
  }
  
  fn add_data(&mut self, data: T) {
    self.sum_tracker.add_data(data);
  }
  
  fn get_average(&self) -> T {
    let sum = self.sum_tracker.get_sum();
    
    
  }
}
