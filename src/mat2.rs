use crate::prelude::is_equal;
use std::ops::{Index, IndexMut};

/// 2 by 2 matrix
///
/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// # use trace::{mat2, mat2::Mat2};
/// assert_eq!(
///    mat2![
///        [1, 2]
///        [5, 6]
///    ],
///    mat2![
///        [1, 2]
///        [5, 6]
///    ]
/// );
/// assert_ne!(
///     mat2![
///         [1, 2]
///         [5, 6]
///     ],
///     mat2![
///         [2, 3]
///         [6, 7]
///     ]
/// );
/// ```
#[derive(Debug)]
pub struct Mat2 {
    elements: Vec<f64>,
}

/// Creates a `Mat2` containing the arguments.
///
/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// # use trace::{mat2, mat2::Mat2};
/// let mat = mat2![
///     [-3, 5]
///     [1, -2]
/// ];
/// ```
#[macro_export]
macro_rules! mat2 {
    [$([$($elem: expr),* $(,)?])*]=>{
	{
	    Mat2::from(&vec![$($(f64::from($elem)),*),*][..])
	}
    }
}

impl Mat2 {
    pub fn zero() -> Self {
        Self {
            elements: vec![0.0_f64; 4],
        }
    }

    pub fn determinant(&self) -> f64 {
        (self[(0, 0)] * self[(1, 1)]) - (self[(0, 1)] * self[(1, 0)])
    }
}

impl PartialEq for Mat2 {
    fn eq(&self, rhs: &Self) -> bool {
        for row in 0..2 {
            for col in 0..2 {
                if !is_equal(self[(row, col)], rhs[(row, col)]) {
                    return false;
                }
            }
        }
        true
    }
}

impl From<&[f64]> for Mat2 {
    fn from(elements: &[f64]) -> Self {
        if elements.len() != 4 {
            panic!("incompatible size for Mat2, size is {}", elements.len());
        }
        Self {
            elements: Vec::from(elements),
        }
    }
}

impl Index<(usize, usize)> for Mat2 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= 2 || index.1 >= 2 {
            panic!(
                "index out of bounds: Mat2 is 2 by 2, index is [({}, {})]",
                index.0, index.1
            );
        }
        &self.elements[index.0 * 2 + index.1]
    }
}

impl IndexMut<(usize, usize)> for Mat2 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= 2 || index.1 >= 2 {
            panic!(
                "index out of bounds: Mat2 is 2 by 2, index is [({}, {})]",
                index.0, index.1
            );
        }
        &mut self.elements[index.0 * 2 + index.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinant() {
        assert!(is_equal(
            mat2![
                [1, 5]
                [-3, 2]
            ]
            .determinant(),
            17.0
        ));
    }
    #[test]
    fn test_index() {
        let mat = mat2![
            [-3, 5]
            [1, -2]
        ];
        assert!(is_equal(mat[(0, 0)], -3.0));
        assert!(is_equal(mat[(0, 1)], 5.0));
        assert!(is_equal(mat[(1, 0)], 1.0));
        assert!(is_equal(mat[(1, 1)], -2.0));
    }
}
