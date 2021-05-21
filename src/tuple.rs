extern crate utility;
use std;

#[derive(Debug,Copy,Clone)]
pub struct Tuple {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32
}

pub type Vector = Tuple;
pub type Point = Tuple;

impl std::cmp::PartialEq for Tuple {
  fn eq (&self, rhs: &Self) -> bool {
    utility::equal(self.x, rhs.x)
      && utility::equal(self.y, rhs.y)
      && utility::equal(self.z, rhs.z)
      && utility::equal(self.w, rhs.w)
  }
}

impl std::ops::Add<Tuple> for Tuple {
  type Output = Self;

  fn add (self, rhs: Tuple) -> Tuple {
    if helpers::is_point(self) && helpers::is_point(rhs) {
      panic!("Can't add a point to a point");
    }

    return helpers::tuple(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w);
  }
}

impl std::ops::Sub<Tuple> for Tuple {
  type Output = Self;

  fn sub (self, rhs: Tuple) -> Tuple {
    if helpers::is_vector(self) && helpers::is_point(rhs) {
      panic!("Can't subtract a point from a vector");
    }

    return Tuple {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w
    }
  }
}

impl std::ops::Mul<f32> for Tuple {
  type Output = Self;

  fn mul (self, rhs: f32) -> Tuple {
    Tuple {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
      w: self.w * rhs
    }
  }
}

impl std::ops::Div<f32> for Tuple {
  type Output = Self;

  fn div (self, rhs: f32) -> Tuple {
    Tuple {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
      w: self.w / rhs
    }
  }
}

impl std::ops::Neg for Tuple {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Tuple {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: -self.w
    }
  }
}

pub mod helpers {
  use super::*;

  pub fn tuple (x: f32, y: f32, z: f32, w: f32) -> Tuple {
    Tuple {
      x: x,
      y: y,
      z: z,
      w: w
    }
  }

  pub fn vector (x: f32, y: f32, z: f32) -> Vector {
    Vector {
      x: x,
      y: y,
      z: z,
      w: 0.0
    }
  }

  pub fn point (x: f32, y: f32, z: f32) -> Point {
    Vector {
      x: x,
      y: y,
      z: z,
      w: 1.0
    }
  }

  pub fn is_vector (tuple: Tuple) -> bool {
    tuple.w == 0.0
  }

  pub fn is_point (tuple: Tuple) -> bool {
    tuple.w == 1.0
  }
}

pub mod ops {
  use super::*;

  pub fn magnitude (a: Tuple) -> f32 {
    ((a.x).powf(2.0) + (a.y).powf(2.0) + (a.z).powf(2.0) + (a.w).powf(2.0)).sqrt()
  }

  pub fn normalize (a: Tuple) -> Tuple {
    a / magnitude(a)
  }

  pub fn dot (a: Tuple, b: Tuple) -> f32 {
    (a.x * b.x) + (a.y * b.y) + (a.z * b.z) + (a.w * b.w)
  }

  pub fn cross (a: Vector, b: Vector) -> Vector {
    helpers::vector(
      (a.y * b.z) - (a.z * b.y),
      (a.z * b.x) - (a.x * b.z),
      (a.x * b.y) - (a.y * b.x)
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tuple_definitions () {
    let p = helpers::tuple(4.3, -4.2, 3.1, 1.0);
    assert!(helpers::is_point(p));
    assert!(!helpers::is_vector(p));

    let v = helpers::tuple(4.3, -4.2, 3.1, 0.0);
    assert!(!helpers::is_point(v));
    assert!(helpers::is_vector(v));
  }

  #[test]
  fn test_point () {
    let p = helpers::point(4.3, -4.2, 3.1);
    assert!(helpers::is_point(p));
    assert!(!helpers::is_vector(p));
    assert_eq!(p.x, 4.3);
    assert_eq!(p.y, -4.2);
    assert_eq!(p.z, 3.1);
    assert_eq!(p.w, 1.0);
  }

  #[test]
  fn test_vector () {
    let v = helpers::vector(4.3, -4.2, 3.1);
    assert!(!helpers::is_point(v));
    assert!(helpers::is_vector(v));
    assert_eq!(v.x, 4.3);
    assert_eq!(v.y, -4.2);
    assert_eq!(v.z, 3.1);
    assert_eq!(v.w, 0.0);
  }

  #[test]
  fn test_add_point_and_vector () {
    // Can add a point and a vector
    let p1 = helpers::point(3.0, -2.0, 5.0);
    let v1 = helpers::vector(-2.0, 3.0, 1.0);
    let p2 = p1 + v1;
    assert!(helpers::is_point(p2));
    assert_eq!(p2, helpers::point(1.0, 1.0, 6.0));
  }

  #[test]
  fn test_add_vectors () {
    // Can add two vectors
    let v1 = helpers::vector(3.0, -2.0, 5.0);
    let v2 = helpers::vector(-2.0, 3.0, 1.0);
    let v3 = v1 + v2;
    assert!(helpers::is_vector(v3));
    assert_eq!(v3, helpers::vector(1.0, 1.0, 6.0));
  }

  #[test]
  #[should_panic]
  fn test_add_two_points () {
    let p1 = helpers::point(3.0, -2.0, 5.0);
    let p2 = helpers::point(3.0, -2.0, 5.0);
    let _result = p1 + p2;
  }

  #[test]
  fn test_subtract_points () {
    let p1 = helpers::point(3.0, 2.0, 1.0);
    let p2 = helpers::point(5.0, 6.0, 7.0);
    let v1 = p1 - p2;
    assert!(helpers::is_vector(v1));
    assert_eq!(v1, helpers::vector(-2.0, -4.0, -6.0));
  }

  #[test]
  fn test_subtract_vectors () {
    let v1 = helpers::vector(3.0, 2.0, 1.0);
    let v2 = helpers::vector(5.0, 6.0, 7.0);
    let v3 = v1 - v2;
    assert!(helpers::is_vector(v3));
    assert_eq!(v3, helpers::vector(-2.0, -4.0, -6.0));
  }

  #[test]
  fn test_subtract_vector_from_point () {
    let p1 = helpers::point(3.0, 2.0, 1.0);
    let v1 = helpers::vector(5.0, 6.0, 7.0);
    let p2 = p1 - v1;
    assert!(helpers::is_point(p2));
    assert_eq!(p2, helpers::point(-2.0, -4.0, -6.0));
  }

  #[test]
  #[should_panic]
  fn test_subtract_point_from_vector () {
    let v1 = helpers::vector(3.0, 2.0, 1.0);
    let p1 = helpers::point(5.0, 6.0, 7.0);
    let _result = v1 - p1;
  }

  #[test]
  fn test_negate_vector () {
    let v1 = helpers::vector(1.0, 2.0, 3.0);
    let v2 = -v1;
    assert!(helpers::is_vector(v2));
    assert_eq!(v2, helpers::vector(-1.0, -2.0, -3.0));
  }

  #[test]
  fn test_multiply () {
    let t1 = helpers::tuple(1.0, -2.0, 3.0, -4.0);
    let t2 = t1 * 3.5;
    assert_eq!(t2, helpers::tuple(3.5, -7.0, 10.5, -14.0));
  }

  #[test]
  fn test_divide () {
    let t1 = helpers::tuple(1.0, -2.0, 3.0, -4.0);
    let t2 = t1 / 2.0;
    assert_eq!(t2, helpers::tuple(0.5, -1.0, 1.5, -2.0));
  }

  #[test]
  fn test_magnitude () {
    assert_eq!(ops::magnitude(helpers::vector(1.0, 0.0, 0.0)), 1.0);
    assert_eq!(ops::magnitude(helpers::vector(0.0, 1.0, 0.0)), 1.0);
    assert_eq!(ops::magnitude(helpers::vector(0.0, 0.0, 1.0)), 1.0);
    assert_eq!(ops::magnitude(helpers::vector(1.0, 2.0, 3.0)), (14.0f32).sqrt());
    assert_eq!(ops::magnitude(helpers::vector(-1.0, -2.0, -3.0)), (14.00f32).sqrt());
  }

  #[test]
  fn test_normalize () {
    let v1 = helpers::vector(4.0, 0.0, 0.0);
    let v2 = helpers::vector(1.0, 0.0, 0.0);
    assert_eq!(ops::normalize(v1), v2);
    let v3 = ops::normalize(helpers::vector(1.0, 2.0, 3.0));
    let v4 = helpers::vector(1.0 / (14.0f32).sqrt(), 2.0 / (14.0f32).sqrt(), 3.0 / (14.0f32).sqrt());
    assert_eq!(v3, v4);
    assert!(utility::equal(ops::magnitude(v3), 1.0));
  }

  #[test]
  fn test_dot () {
    let v1 = helpers::vector(1.0, 2.0, 3.0);
    let v2 = helpers::vector(2.0, 3.0, 4.0);
    assert_eq!(ops::dot(v1, v2), 20.0);
  }

  #[test]
  fn test_cross () {
    let v1 = helpers::vector(1.0, 2.0, 3.0);
    let v2 = helpers::vector(2.0, 3.0, 4.0);
    assert_eq!(ops::cross(v1, v2), helpers::vector(-1.0, 2.0, -1.0));
    assert_eq!(ops::cross(v2, v1), helpers::vector(1.0, -2.0, 1.0));
  }
}
