use array2d::Array2D;
use crate::tuple;
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

    for row in 0..4 {
      for column in 0..4 {
        result[(row, column)] = (0..4).map(|i| -> f32 {
          self[(row, i)] * rhs[(i, column)]
        }).sum();
      }
    }

    return Matrix {
      values: result
    };
  }
}

impl std::ops::Mul<Tuple> for Matrix {
  type Output = Tuple;

  fn mul (self, rhs: Tuple) -> Tuple {
    let mut result = tuple::helpers::tuple(0.0, 0.0, 0.0, 0.0);

    for row_index in 0..4 {
      result[row_index] = (0..4).map(|i| -> f32 {
        self[(row_index, i)] * rhs[i]
      }).sum();
    }

    return result;
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

  pub fn identity () -> Matrix {
    Matrix {
      values: Array2D::from_rows(&vec![
        vec![1.0, 0.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0]
      ])
    }
  }
}

pub mod ops {
  use super::*;

  pub fn transpose (matrix: Matrix) -> Matrix {
    Matrix {
      values: Array2D::from_columns(&matrix.values.as_rows())
    }
  }

  pub fn determinant (array: Array2D<f32>) -> f32 {
    if array.num_rows() != 2 && array.num_columns() != 2 {
      panic!("Can only find determinant for 2x2 matrix");
    }

    return array[(0, 0)] * array[(1, 1)] - array[(0, 1)] * array[(1, 0)];
  }

  // TODO: rewrite using array subviews or slices
  pub fn submatrix (array: Array2D<f32>, remove_row: usize, remove_column: usize) -> Array2D<f32> {
    let array_rows = array.num_rows();
    let array_columns = array.num_columns();
    let mut result = Array2D::filled_with(0.0, array_rows - 1, array_columns - 1);

    for row in 0..array_rows {
      if row != remove_row {
        for column in 0..array_columns {
          if column != remove_column {
            let row_index = if row < remove_row { row } else { row - 1 };
            let column_index = if column < remove_column { column } else { column - 1};

            result[(row_index, column_index)] = array[(row, column)];
          }
        }
      }
    }

    return result;
  }
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

  #[test]
  fn test_matrix_identity () {
    let m1 = helpers::matrix(vec![
      vec![1.0, 2.0, 3.0, 4.0],
      vec![5.0, 6.0, 7.0, 8.0],
      vec![9.0, 8.0, 7.0, 6.0],
      vec![5.0, 4.0, 3.0, 2.0]
    ]);
    let m2 = helpers::identity();
    let m3 = m1 * m2;

    assert_eq!(m3, helpers::matrix(vec![
      vec![1.0, 2.0, 3.0, 4.0],
      vec![5.0, 6.0, 7.0, 8.0],
      vec![9.0, 8.0, 7.0, 6.0],
      vec![5.0, 4.0, 3.0, 2.0]
    ]));
  }

  #[test]
  fn test_matrix_tuple_multiplication () {
    let m1 = helpers::matrix(vec![
      vec![1.0, 2.0, 3.0, 4.0],
      vec![2.0, 4.0, 4.0, 2.0],
      vec![8.0, 6.0, 4.0, 1.0],
      vec![0.0, 0.0, 0.0, 1.0]
    ]);
    let t1 = tuple::helpers::tuple(1.0, 2.0, 3.0, 1.0);
    let t2 = m1 * t1;
    assert_eq!(t2, tuple::helpers::tuple(18.0, 24.0, 33.0, 1.0));
  }

  #[test]
  fn test_tuple_identity () {
    let m1 = helpers::identity();
    let t1 = tuple::helpers::tuple(1.0, 2.0, 3.0, 1.0);
    let t2 = t1.clone();
    let t3 = m1 * t1;
    assert_eq!(t2, t3);
  }

  #[test]
  fn test_transpose_matrix () {
    let m1 = helpers::matrix(vec![
      vec![0.0, 9.0, 3.0, 0.0],
      vec![9.0, 8.0, 0.0, 8.0],
      vec![1.0, 8.0, 5.0, 3.0],
      vec![0.0, 0.0, 5.0, 8.0]
    ]);

    let m2 = ops::transpose(m1);

    let m3 = helpers::matrix(vec![
      vec![0.0, 9.0, 1.0, 0.0],
      vec![9.0, 8.0, 8.0, 0.0],
      vec![3.0, 0.0, 5.0, 5.0],
      vec![0.0, 8.0, 3.0, 8.0]
    ]);

    assert_eq!(m2, m3);
  }

  #[test]
  fn test_determinant () {
    let m1 = Array2D::from_rows(&vec![
      vec![1.0, 5.0],
      vec![-3.0, 2.0]
    ]);

    assert_eq!(ops::determinant(m1), 17.0);
  }

  #[test]
  fn test_submatrix () {
    let m1 = Array2D::from_rows(&vec![
      vec![1.0, 5.0, 0.0],
      vec![-3.0, 2.0, 7.0],
      vec![0.0, 6.0, -3.0]
    ]);

    let m2 = Array2D::from_rows(&vec![
      vec![-3.0, 2.0],
      vec![0.0, 6.0]
    ]);

    assert_eq!(ops::submatrix(m1, 0, 2), m2);

    let m3 = Array2D::from_rows(&vec![
      vec![-6.0, 1.0, 1.0, 6.0],
      vec![-8.0, 5.0, 8.0, 6.0],
      vec![-1.0, 0.0, 8.0, 2.0],
      vec![-7.0, 1.0, -1.0, 1.0]
    ]);

    let m4 = Array2D::from_rows(&vec![
      vec![-6.0, 1.0, 6.0],
      vec![-8.0, 8.0, 6.0],
      vec![-7.0, -1.0, 1.0]
    ]);

    assert_eq!(ops::submatrix(m3, 2, 1), m4);
  }
}
