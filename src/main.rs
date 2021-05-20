extern crate image;
use std::ops;
use std::cmp;
use image::{ImageBuffer, Rgb};

const EPSILON: f32 = 0.00001;

fn equal (a: f32, b: f32) -> bool {
  ((a - b).abs()) < EPSILON
}

#[derive(Debug,Copy,Clone)]
struct Tuple {
  x: f32,
  y: f32,
  z: f32,
  w: f32
}
type Vector = Tuple;
type Point = Tuple;

impl cmp::PartialEq for Tuple {
  fn eq (&self, rhs: &Self) -> bool {
    equal(self.x, rhs.x)
      && equal(self.y, rhs.y)
      && equal(self.z, rhs.z)
      && equal(self.w, rhs.w)
  }
}

#[derive(Debug,Copy,Clone)]
struct Color {
  red: f32,
  green: f32,
  blue: f32
}

impl cmp::PartialEq for Color {
  fn eq (&self, rhs: &Self) -> bool {
    equal(self.red, rhs.red)
      && equal(self.green, rhs.green)
      && equal(self.blue, rhs.blue)
  }
}

impl ops::Add<Color> for Color {
  type Output = Self;

  fn add (self, rhs: Color) -> Color {
    Color {
      red: self.red + rhs.red,
      green: self.green + rhs.green,
      blue: self.blue + rhs.blue
    }
  }
}

impl ops::Sub<Color> for Color {
  type Output = Self;

  fn sub (self, rhs: Color) -> Color {
    Color {
      red: self.red - rhs.red,
      green: self.green - rhs.green,
      blue: self.blue - rhs.blue
    }
  }
}

impl ops::Mul<f32> for Color {
  type Output = Self;

  fn mul (self, rhs: f32) -> Color {
    Color {
      red: self.red * rhs,
      green: self.green * rhs,
      blue: self.blue * rhs
    }
  }
}

impl ops::Mul<Color> for Color {
  type Output = Self;

  fn mul(self, rhs: Color) -> Color {
    Color {
      red: self.red * rhs.red,
      green: self.green * rhs.green,
      blue: self.blue * rhs.blue
    }
  }
}

fn color (red: f32, green: f32, blue: f32) -> Color {
  Color {
    red: red,
    green: green,
    blue: blue
  }
}

fn tuple (x: f32, y: f32, z: f32, w: f32) -> Tuple {
  Tuple {
    x: x,
    y: y,
    z: z,
    w: w
  }
}

fn vector (x: f32, y: f32, z: f32) -> Vector {
  Vector {
    x: x,
    y: y,
    z: z,
    w: 0.0
  }
}

fn point (x: f32, y: f32, z: f32) -> Point {
  Vector {
    x: x,
    y: y,
    z: z,
    w: 1.0
  }
}

fn write_pixel(canvas: &mut ImageBuffer<Rgb<u8>, Vec<Rgb<u8>>>, x: u32, y: u32, color: &Color) {
  canvas.put_pixel(x, y, normalize_color(color))
}

fn is_vector (tuple: Tuple) -> bool {
  tuple.w == 0.0
}

fn is_point (tuple: Tuple) -> bool {
  tuple.w == 1.0
}

impl ops::Add<Tuple> for Tuple {
  type Output = Self;

  fn add (self, rhs: Tuple) -> Tuple {
    if is_point(self) && is_point(rhs) {
      panic!("Can't add a point to a point");
    }

    return tuple(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w);
  }
}

impl ops::Sub<Tuple> for Tuple {
  type Output = Self;

  fn sub (self, rhs: Tuple) -> Tuple {
    if is_vector(self) && is_point(rhs) {
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

impl ops::Mul<f32> for Tuple {
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

impl ops::Div<f32> for Tuple {
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

impl ops::Neg for Tuple {
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

fn magnitude (a: Tuple) -> f32 {
  ((a.x).powf(2.0) + (a.y).powf(2.0) + (a.z).powf(2.0) + (a.w).powf(2.0)).sqrt()
}

fn normalize (a: Tuple) -> Tuple {
  a / magnitude(a)
}

fn normalize_color_value (value: f32) -> u8 {
  let normalized_value = value * 255.0;

  if normalized_value > 255.0 {
    return 255;
  }

  if normalized_value < 0.0 {
    return 0;
  }

  return (value * 255.0) as u8;
}

fn normalize_color (color: &Color) -> Rgb<u8> {
  image::Rgb([normalize_color_value(color.red), normalize_color_value(color.green), normalize_color_value(color.blue)])
}

fn dot (a: Tuple, b: Tuple) -> f32 {
  (a.x * b.x) + (a.y * b.y) + (a.z * b.z) + (a.w * b.w)
}

fn cross (a: Vector, b: Vector) -> Vector {
  vector(
    (a.y * b.z) - (a.z * b.y),
    (a.z * b.x) - (a.x * b.z),
    (a.x * b.y) - (a.y * b.x)
  )
}

struct Environment {
  gravity: Vector,
  wind: Vector
}

struct Projectile {
  position: Point,
  velocity: Vector
}

fn tick (environment: &Environment, projectile: Projectile) -> Projectile {
  Projectile {
    position: projectile.position + projectile.velocity,
    velocity: projectile.velocity + environment.gravity + environment.wind
  }
}

fn main () {
  let mut p = Projectile {
    position: point(0.0, 1.0, 0.0),
    velocity: normalize(vector(1.0, 1.0, 0.0))
  };

  let e = Environment {
    gravity: vector(0.0, -0.1, 0.0),
    wind: vector(-0.01, 0.0, 0.0)
  };

  while p.position.y > 0.0 {
    p = tick(&e, p);
    println!("{:?}, {:?}, {:?}", p.position.x, p.position.y, p.position.z);
  }
}

mod tests {
  use super::*;

  #[test]
  fn test_tuple_definitions () {
    let p = tuple(4.3, -4.2, 3.1, 1.0);
    assert!(is_point(p));
    assert!(!is_vector(p));

    let v = tuple(4.3, -4.2, 3.1, 0.0);
    assert!(!is_point(v));
    assert!(is_vector(v));
  }

  #[test]
  fn test_point () {
    let p = point(4.3, -4.2, 3.1);
    assert!(is_point(p));
    assert!(!is_vector(p));
    assert_eq!(p.x, 4.3);
    assert_eq!(p.y, -4.2);
    assert_eq!(p.z, 3.1);
    assert_eq!(p.w, 1.0);
  }

  #[test]
  fn test_vector () {
    let v = vector(4.3, -4.2, 3.1);
    assert!(!is_point(v));
    assert!(is_vector(v));
    assert_eq!(v.x, 4.3);
    assert_eq!(v.y, -4.2);
    assert_eq!(v.z, 3.1);
    assert_eq!(v.w, 0.0);
  }

  #[test]
  fn test_equal () {
    assert!(equal(2.1, 2.1));
    assert!(equal(2.1, 2.100001));
    assert!(!equal(2.0, 2.1));
    assert!(!equal(2.1, 2.1001));
  }

  #[test]
  fn test_add_point_and_vector () {
    // Can add a point and a vector
    let p1 = point(3.0, -2.0, 5.0);
    let v1 = vector(-2.0, 3.0, 1.0);
    let p2 = p1 + v1;
    assert!(is_point(p2));
    assert_eq!(p2, point(1.0, 1.0, 6.0));
  }

  #[test]
  fn test_add_vectors () {
    // Can add two vectors
    let v1 = vector(3.0, -2.0, 5.0);
    let v2 = vector(-2.0, 3.0, 1.0);
    let v3 = v1 + v2;
    assert!(is_vector(v3));
    assert_eq!(v3, vector(1.0, 1.0, 6.0));
  }

  #[test]
  #[should_panic]
  fn test_add_two_points () {
    let p1 = point(3.0, -2.0, 5.0);
    let p2 = point(3.0, -2.0, 5.0);
    let _result = p1 + p2;
  }

  #[test]
  fn test_subtract_points () {
    let p1 = point(3.0, 2.0, 1.0);
    let p2 = point(5.0, 6.0, 7.0);
    let v1 = p1 - p2;
    assert!(is_vector(v1));
    assert_eq!(v1, vector(-2.0, -4.0, -6.0));
  }

  #[test]
  fn test_subtract_vectors () {
    let v1 = vector(3.0, 2.0, 1.0);
    let v2 = vector(5.0, 6.0, 7.0);
    let v3 = v1 - v2;
    assert!(is_vector(v3));
    assert_eq!(v3, vector(-2.0, -4.0, -6.0));
  }

  #[test]
  fn test_subtract_vector_from_point () {
    let p1 = point(3.0, 2.0, 1.0);
    let v1 = vector(5.0, 6.0, 7.0);
    let p2 = p1 - v1;
    assert!(is_point(p2));
    assert_eq!(p2, point(-2.0, -4.0, -6.0));
  }

  #[test]
  #[should_panic]
  fn test_subtract_point_from_vector () {
    let v1 = vector(3.0, 2.0, 1.0);
    let p1 = point(5.0, 6.0, 7.0);
    let _result = v1 - p1;
  }

  #[test]
  fn test_negate_vector () {
    let v1 = vector(1.0, 2.0, 3.0);
    let v2 = -v1;
    assert!(is_vector(v2));
    assert_eq!(v2, vector(-1.0, -2.0, -3.0));
  }

  #[test]
  fn test_multiply () {
    let t1 = tuple(1.0, -2.0, 3.0, -4.0);
    let t2 = t1 * 3.5;
    assert_eq!(t2, tuple(3.5, -7.0, 10.5, -14.0));
  }

  #[test]
  fn test_divide () {
    let t1 = tuple(1.0, -2.0, 3.0, -4.0);
    let t2 = t1 / 2.0;
    assert_eq!(t2, tuple(0.5, -1.0, 1.5, -2.0));
  }

  #[test]
  fn test_magnitude () {
    assert_eq!(magnitude(vector(1.0, 0.0, 0.0)), 1.0);
    assert_eq!(magnitude(vector(0.0, 1.0, 0.0)), 1.0);
    assert_eq!(magnitude(vector(0.0, 0.0, 1.0)), 1.0);
    assert_eq!(magnitude(vector(1.0, 2.0, 3.0)), (14.0f32).sqrt());
    assert_eq!(magnitude(vector(-1.0, -2.0, -3.0)), (14.00f32).sqrt());
  }

  #[test]
  fn test_normalize () {
    let v1 = vector(4.0, 0.0, 0.0);
    let v2 = vector(1.0, 0.0, 0.0);
    assert_eq!(normalize(v1), v2);
    let v3 = normalize(vector(1.0, 2.0, 3.0));
    let v4 = vector(1.0 / (14.0f32).sqrt(), 2.0 / (14.0f32).sqrt(), 3.0 / (14.0f32).sqrt());
    assert_eq!(v3, v4);
    assert!(equal(magnitude(v3), 1.0));
  }

  #[test]
  fn test_dot () {
    let v1 = vector(1.0, 2.0, 3.0);
    let v2 = vector(2.0, 3.0, 4.0);
    assert_eq!(dot(v1, v2), 20.0);
  }

  #[test]
  fn test_cross () {
    let v1 = vector(1.0, 2.0, 3.0);
    let v2 = vector(2.0, 3.0, 4.0);
    assert_eq!(cross(v1, v2), vector(-1.0, 2.0, -1.0));
    assert_eq!(cross(v2, v1), vector(1.0, -2.0, 1.0));
  }

  #[test]
  fn test_color_definition () {
    let c1 = color(-0.5, 0.4, 1.7);

    assert_eq!(c1.red, -0.5);
    assert_eq!(c1.green, 0.4);
    assert_eq!(c1.blue, 1.7);
  }

  #[test]
  fn test_add_colors () {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    let c3 = c1 + c2;
    assert_eq!(c3, color(1.6, 0.7, 1.0));
  }

  #[test]
  fn test_subtract_colors () {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    let c3 = c1 - c2;
    assert_eq!(c3, color(0.2, 0.5, 0.5));
  }

  #[test]
  fn test_multiply_color_scalar () {
    let c1 = color(0.2, 0.3, 0.4);
    let c2 = c1 * 2.0;
    assert_eq!(c2, color(0.4, 0.6, 0.8));
  }

  #[test]
  fn test_multiple_colors () {
    let c1 = color(1.0, 0.2, 0.4);
    let c2 = color(0.9, 1.0, 0.1);
    let c3 = c1 * c2;
    assert_eq!(c3, color(0.9, 0.2, 0.04));
  }

  #[test]
  fn test_write_pixel () {
    let mut canvas = ImageBuffer::new(10, 20);
    let red = color(1.0, 0.0, 0.0);
    write_pixel(&mut canvas, 2, 3, &red);
  }
}
