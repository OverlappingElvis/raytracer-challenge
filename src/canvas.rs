use array2d::Array2D;
use crate::color::helpers::color;
use crate::color::Color;

pub type Canvas = Array2D<Color>;

pub mod helpers {
  use super::*;

  pub fn canvas (x: usize, y: usize) -> Canvas {
    Array2D::filled_with(color(0.0, 0.0, 0.0), x, y)
  }
}
