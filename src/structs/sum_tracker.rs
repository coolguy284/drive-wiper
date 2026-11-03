use core::array::from_fn;
use std::ops::{AddAssign, SubAssign};

use num::Zero;

// Tracks sum of datapoints in a circular buffer
// https://www.geeksforgeeks.org/rust/rust-multiple-bounds/
pub(crate) struct SumTracker<T: Clone + AddAssign + SubAssign + Zero, const DATA_POINT_LENGTH: usize> {
  data_points: [T; DATA_POINT_LENGTH],
  data_point_index: usize,
  sum: T,
}

impl<T: Clone + AddAssign + SubAssign + Zero, const DATA_POINT_LENGTH: usize> SumTracker<T, DATA_POINT_LENGTH> {
  pub(crate) fn new() -> Self {
    // https://stackoverflow.com/questions/76254481/how-do-i-limit-the-values-allowed-in-a-generic-const-parameter/76254798#76254798
    const { assert!(DATA_POINT_LENGTH > 0, "data_point_length must be positive") };
    
    SumTracker {
      data_points: from_fn(|_| T::zero()),
      data_point_index: 0,
      sum: T::zero(),
    }
  }
  
  pub(crate) fn add_data(&mut self, data: T) {
    self.sum -= self.data_points[self.data_point_index].clone();
    self.data_points[self.data_point_index] = data.clone();
    self.sum += data.clone();
    self.data_point_index += 1;
    self.data_point_index %= DATA_POINT_LENGTH;
  }
  
  pub(crate) fn get_sum(&self) -> T {
    self.sum.clone()
  }
}
