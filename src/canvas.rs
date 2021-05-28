extern crate image;
use array2d::Array2D;
use crate::color::helpers::color;
use crate::color::ops::normalize;
use crate::color::Color;
use image::{ImageBuffer, RgbImage, Rgb, imageops};

pub type Canvas = Array2D<Color>;

pub mod helpers {
  use super::*;

  pub fn canvas (x: usize, y: usize) -> Canvas {
    Array2D::filled_with(color(0.0, 0.0, 0.0), x, y)
  }

  pub fn to_image (canvas: Canvas) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(canvas.num_rows() as u32, canvas.num_columns() as u32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
      let color = normalize(&canvas[(x as usize, y as usize)]);

      *pixel = Rgb([color.red as u8, color.green as u8, color.blue as u8]);
    }

    return imageops::flip_vertical(&img);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_canvas () {
    let c1 = helpers::canvas(10, 10);
    assert_eq!(c1.num_rows(), 10);
    assert_eq!(c1.num_columns(), 10);
    assert_eq!(c1[(2, 3)], color(0.0, 0.0, 0.0));
  }
}
