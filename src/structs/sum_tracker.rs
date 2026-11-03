struct SumTracker<T, const data_point_length: usize> {
  data_points: [T; data_point_length],
  data_point_index: usize,
  sum: T,
}

impl<T, const data_point_length: usize> SumTracker<T, data_point_length> {
  fn new() -> Self {
    SumTracker {
      data_points: (),
      data_point_index: 0,
      sum: ()
    }
  }
  
  fn add_data(&mut self, data: T) {
    self.sum -= self.data_points[self.data_point_index];
    self.data_points[self.data_point_index] = data;
    self.sum += data;
    data_point_index += 1;
    data_point_index %= data_point_length;
  }
  
  fn get_sum(&self) -> T {
    self.sum
  }
}
