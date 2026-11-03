struct SumTracker<T, const data_point_length: usize> {
  data_points: [T; data_point_length],
  data_point_index: usize,
  sum: T,
}

impl<T, const data_point_length: usize> SumTracker<T, data_point_length> {
  fn new() -> Self {
    SumTracker {
      data_points: (),
      data_point_index: (),
      sum: ()
    }
  }
  
  fn add_data(data: T) {
    
  }
  
  fn get_sum() -> T {
    
  }
}
