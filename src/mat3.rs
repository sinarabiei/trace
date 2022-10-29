use crate::mat2::Mat2;
use crate::prelude::is_equal;
use std::ops::{Index, IndexMut};

/// 3 by 3 matrix
///
/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// # use trace::{mat3, mat3::Mat3};
/// assert_eq!(
///     mat3![
///         [1, 2, 3]
///         [5, 6, 7]
///         [9, 8, 7]
///     ],
///     mat3![
///         [1, 2, 3]
///         [5, 6, 7]
///         [9, 8, 7]
///     ]
/// );
/// assert_ne!(
///     mat3![
///         [1, 2, 3]
///         [5, 6, 7]
///         [9, 8, 7]
///     ],
///     mat3![
///         [2, 3, 4]
///         [6, 7, 8]
///         [8, 7, 6]
///     ]
/// );
/// ```
#[derive(Debug)]
pub struct Mat3 {
    elements: Vec<f64>,
}

/// Creates a `Mat3` containing the arguments.
///
/// # Example
///
/// ```
/// # use trace::prelude::*;
/// # use trace::{mat3, mat3::Mat3};
/// let mat = mat3![
///     [-3, 5, 0]
///     [1, -2, -7]
///     [0, 1, 1]
/// ];
/// ```
#[macro_export]
macro_rules! mat3 {
    [$([$($elem: expr),* $(,)?])*]=>{
	{
	    Mat3::from(&vec![$($(f64::from($elem)),*),*][..])
	}
    }
}

impl Mat3 {
    pub fn zero() -> Self {
        Self {
            elements: vec![0.0_f64; 9],
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Mat2 {
        let mut elements = Vec::new();
        for r in 0..3 {
            for c in 0..3 {
                if r != row && c != col {
                    elements.push(self[(r, c)]);
                }
            }
        }
        Mat2::from(&elements[..])
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        (-1_i8).pow((row + col).try_into().unwrap()) as f64 * self.minor(row, col)
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for col in 0..3 {
            det += self[(0, col)] * self.cofactor(0, col)
        }
        det
    }
}

impl PartialEq for Mat3 {
    fn eq(&self, rhs: &Self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if !is_equal(self[(row, col)], rhs[(row, col)]) {
                    return false;
                }
            }
        }
        true
    }
}

impl From<&[f64]> for Mat3 {
    fn from(elements: &[f64]) -> Self {
        if elements.len() != 9 {
            panic!("incompatible size for Mat3, size is {}", elements.len());
        }
        Self {
            elements: Vec::from(elements),
        }
    }
}

impl Index<(usize, usize)> for Mat3 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= 3 || index.1 >= 3 {
            panic!(
                "index out of bounds: Mat3 is 3 by 3, index is [({}, {})]",
                index.0, index.1
            );
        }
        &self.elements[index.0 * 3 + index.1]
    }
}

impl IndexMut<(usize, usize)> for Mat3 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= 3 || index.1 >= 3 {
            panic!(
                "index out of bounds: Mat3 is 3 by 3, index is [({}, {})]",
                index.0, index.1
            );
        }
        &mut self.elements[index.0 * 3 + index.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mat2;

    #[test]
    fn test_submatrix() {
        assert_eq!(
            mat3![
                [1, 5, 0]
                [-3, 2, 7]
                [0, 6, -3]
            ]
            .submatrix(0, 2),
            mat2![
                [-3, 2]
                [0, 6]
            ]
        );
    }

    #[test]
    fn test_minor() {
        assert!(is_equal(
            mat3![
                [3, 5, 0]
                [2, -1, -7]
                [6, -1, 5]
            ]
            .minor(1, 0),
            25.0
        ));
    }

    #[test]
    fn test_cofactor() {
        let mat = mat3![
            [3, 5, 0]
            [2, -1, -7]
            [6, -1, 5]
        ];
        assert!(is_equal(mat.cofactor(0, 0), -12.0));
        assert!(is_equal(mat.cofactor(1, 0), -25.0));
    }

    #[test]
    fn test_determinant() {
        assert!(is_equal(
            mat3![
                [1, 2, 6]
                [-5, 8, -4]
                [2, 6, 4]
            ]
            .determinant(),
            -196.0
        ));
    }

    #[test]
    fn test_index() {
        let mat = mat3![
            [-3, 5, 0]
            [1, -2, -7]
            [0, 1, 1]
        ];
        assert!(is_equal(mat[(0, 0)], -3.0));
        assert!(is_equal(mat[(1, 1)], -2.0));
        assert!(is_equal(mat[(2, 2)], 1.0));
    }
}
