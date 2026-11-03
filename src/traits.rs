use std::time::Duration;

pub(crate) trait Zero {
  fn zero() -> Self;
}

impl Zero for u64 {
  fn zero() -> Self {
    0
  }
}

impl Zero for Duration {
  fn zero() -> Self {
    Self::ZERO
  }
}
