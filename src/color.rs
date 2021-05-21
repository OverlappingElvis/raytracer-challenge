extern crate utility;
use std;

#[derive(Debug,Copy,Clone)]
pub struct Color {
  red: f32,
  green: f32,
  blue: f32
}

impl std::cmp::PartialEq for Color {
  fn eq (&self, rhs: &Self) -> bool {
    utility::equal(self.red, rhs.red)
      && utility::equal(self.green, rhs.green)
      && utility::equal(self.blue, rhs.blue)
  }
}

impl std::ops::Add<Color> for Color {
  type Output = Self;

  fn add (self, rhs: Color) -> Color {
    Color {
      red: self.red + rhs.red,
      green: self.green + rhs.green,
      blue: self.blue + rhs.blue
    }
  }
}

impl std::ops::Sub<Color> for Color {
  type Output = Self;

  fn sub (self, rhs: Color) -> Color {
    Color {
      red: self.red - rhs.red,
      green: self.green - rhs.green,
      blue: self.blue - rhs.blue
    }
  }
}

impl std::ops::Mul<f32> for Color {
  type Output = Self;

  fn mul (self, rhs: f32) -> Color {
    Color {
      red: self.red * rhs,
      green: self.green * rhs,
      blue: self.blue * rhs
    }
  }
}

impl std::ops::Mul<Color> for Color {
  type Output = Self;

  fn mul(self, rhs: Color) -> Color {
    Color {
      red: self.red * rhs.red,
      green: self.green * rhs.green,
      blue: self.blue * rhs.blue
    }
  }
}

pub mod helpers {
  pub fn color (red: f32, green: f32, blue: f32) -> super::Color {
    super::Color {
      red: red,
      green: green,
      blue: blue
    }
  }
}

pub mod ops {

  fn normalize_color_value (value: f32) -> f32 {
    let normalized_value = value * 255.0;

    if normalized_value > 255.0 {
      return 255.0;
    }

    if normalized_value < 0.0 {
      return 0.0;
    }

    return value * 255.0;
  }

  pub fn normalize (color: &super::Color) -> super::Color {
    super::helpers::color(self::normalize_color_value(color.red), self::normalize_color_value(color.green), self::normalize_color_value(color.blue))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_color_definition () {
    let c1 = helpers::color(-0.5, 0.4, 1.7);

    assert_eq!(c1.red, -0.5);
    assert_eq!(c1.green, 0.4);
    assert_eq!(c1.blue, 1.7);
  }

  #[test]
  fn test_add_colors () {
    let c1 = helpers::color(0.9, 0.6, 0.75);
    let c2 = helpers::color(0.7, 0.1, 0.25);
    let c3 = c1 + c2;
    assert_eq!(c3, helpers::color(1.6, 0.7, 1.0));
  }

  #[test]
  fn test_subtract_colors () {
    let c1 = helpers::color(0.9, 0.6, 0.75);
    let c2 = helpers::color(0.7, 0.1, 0.25);
    let c3 = c1 - c2;
    assert_eq!(c3, helpers::color(0.2, 0.5, 0.5));
  }

  #[test]
  fn test_multiply_color_scalar () {
    let c1 = helpers::color(0.2, 0.3, 0.4);
    let c2 = c1 * 2.0;
    assert_eq!(c2, helpers::color(0.4, 0.6, 0.8));
  }

  #[test]
  fn test_multiply_colors () {
    let c1 = helpers::color(1.0, 0.2, 0.4);
    let c2 = helpers::color(0.9, 1.0, 0.1);
    let c3 = c1 * c2;
    assert_eq!(c3, helpers::color(0.9, 0.2, 0.04));
  }
}

