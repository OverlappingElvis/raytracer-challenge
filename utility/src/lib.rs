const EPSILON: f32 = 0.00001;

pub fn equal (a: f32, b: f32) -> bool {
  ((a - b).abs()) < EPSILON
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_equal () {
    assert!(equal(2.1, 2.1));
    assert!(equal(2.1, 2.100001));
    assert!(!equal(2.0, 2.1));
    assert!(!equal(2.1, 2.1001));
  }
}
