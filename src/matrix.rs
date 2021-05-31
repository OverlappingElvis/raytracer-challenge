use array2d::Array2D;
use crate::tuple::Tuple;
use std;

#[derive(Debug)]
pub struct Matrix {
  values: Array2D<f32>
}

impl std::cmp::PartialEq for Matrix {
  fn eq (&self, rhs: &Self) -> bool {
    self.values == rhs.values
  }
}

impl std::ops::Mul<Matrix> for Matrix {
  type Output = Self;

  fn mul (self, rhs: Matrix) -> Matrix {
    let mut result = Array2D::filled_with(0.0, self.values.num_rows(), self.values.num_columns());

    let rows = self.values.num_rows();
    let columns = self.values.num_columns();

    for row in 0..rows {
      for column in 0..columns {

        result[(row, column)] = (0..rows).map(|x| -> f32 { self[(row, x)] * rhs[(x, column)] }).sum();
      }
    }

    return Matrix {
      values: result
    };
  }
}

impl std::ops::Index<(usize, usize)> for Matrix {
  type Output = f32;

  fn index(&self, coords: (usize, usize)) -> &Self::Output {
    &self.values[(coords.0, coords.1)]
  }
}

pub mod helpers {
  use super::*;

  pub fn matrix (rows: Vec<Vec<f32>>) -> Matrix {
    Matrix {
      values: Array2D::from_rows(&rows)
    }
  }
}

pub mod ops {

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_matrix () {
    let m = helpers::matrix(vec![
      vec![1.0, 2.0, 3.0, 4.0],
      vec![5.5, 6.5, 7.5, 8.5],
      vec![9.0, 10.0, 11.0, 12.0],
      vec![13.5, 14.5, 15.5, 16.5]
    ]);

    assert_eq!(m[(0, 3)], 4.0);
    assert_eq!(m[(1, 0)], 5.5);
    assert_eq!(m[(1, 1)], 6.5);
    assert_eq!(m[(2, 1)], 10.0);

    let m1 = helpers::matrix(vec![
      vec![-3.0, 5.0],
      vec![1.0, -2.0]
    ]);

    assert_eq!(m1[(0, 0)], -3.0);
    assert_eq!(m1[(0, 1)], 5.0);
    assert_eq!(m1[(1, 0)], 1.0);
    assert_eq!(m1[(1, 1)], -2.0);
  }

  #[test]
  fn test_matrix_equality () {
    let rows = vec![
      vec![1.0, 2.0, 3.0, 4.0],
      vec![5.5, 6.5, 7.5, 8.5],
      vec![9.0, 10.0, 11.0, 12.0],
      vec![13.5, 14.5, 15.5, 16.5]
    ];

    let m1 = helpers::matrix(rows.clone());
    let m2 = helpers::matrix(rows.clone());

    assert_eq!(m1, m2);
  }

  #[test]
  fn test_matrix_multiplication () {
    let m1 = helpers::matrix(vec![
      vec![1.0, 2.0, 3.0, 4.0],
      vec![5.0, 6.0, 7.0, 8.0],
      vec![9.0, 8.0, 7.0, 6.0],
      vec![5.0, 4.0, 3.0, 2.0]
    ]);
  let m2 = helpers::matrix(vec![
      vec![-2.0, 1.0, 2.0, 3.0],
      vec![3.0, 2.0, 1.0, -1.0],
      vec![4.0, 3.0, 6.0, 5.0],
      vec![1.0, 2.0, 7.0, 8.0]
    ]);
    let m3 = helpers::matrix(vec![
      vec![20.0, 22.0, 50.0, 48.0],
      vec![44.0, 54.0, 114.0, 108.0],
      vec![40.0, 58.0, 110.0, 102.0],
      vec![16.0, 26.0, 46.0, 42.0]
    ]);
    assert_eq!(m1 * m2, m3);
  }
}
