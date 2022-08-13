use crate::point::Point;
use crate::prelude::is_equal;
use crate::tuple::Tuple;
use crate::vector::Vector;
use std::ops::{Index, IndexMut, Mul};

/// 4 by 4 matrix
///
/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(
///     mat4![
///         [1, 2, 3, 4]
///         [5, 6, 7, 8]
///         [9, 8, 7, 6]
///         [5, 4, 3, 2]
///     ],
///     mat4![
///         [1, 2, 3, 4]
///         [5, 6, 7, 8]
///         [9, 8, 7, 6]
///         [5, 4, 3, 2]
///     ]
/// );
/// assert_ne!(
///     mat4![
///         [1, 2, 3, 4]
///         [5, 6, 7, 8]
///         [9, 8, 7, 6]
///         [5, 4, 3, 2]
///     ],
///     mat4![
///         [2, 3, 4, 5]
///         [6, 7, 8, 9]
///         [8, 7, 6, 5]
///         [4, 3, 2, 1]
///     ]
/// );
/// ```
#[derive(Debug)]
pub struct Mat4 {
    elements: Vec<f64>,
}

/// Creates a `Mat4` containing the arguments.
///
/// # Example
///
/// ```
/// # use trace::prelude::*;
/// let mat = mat4![
///     [1, 2, 3, 4]
///     [5.5, 6.5, 7.5, 8.5]
///     [9, 10, 11, 12]
///     [13.5, 14.5, 15.5, 16.5]
/// ];
/// ```
#[macro_export]
macro_rules! mat4 {
    [$([$($elem: expr),* $(,)?])*]=>{
	{
	    Mat4::from(&vec![$($(f64::from($elem)),*),*][..])
	}
    }
}

impl Mat4 {
    pub fn zero() -> Self {
        Self {
            elements: vec![0.0_f64; 16],
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// // Multiplying a matrix by the identity matrix
    /// let mat = mat4![
    ///     [0, 1, 2, 4]
    ///     [1, 2, 4, 8]
    ///     [2, 4, 8, 16]
    ///     [4, 8, 16, 32]
    /// ];
    /// assert_eq!(mat.clone() * Mat4::identity(), mat);
    ///
    /// // Multiplying the identity matrix by a tuple
    /// let tuple = tuple![1, 2, 3, 4];
    /// assert_eq!(Mat4::identity() * tuple, tuple);
    /// ```
    pub fn identity() -> Self {
        mat4![
            [1, 0, 0, 0]
            [0, 1, 0, 0]
            [0, 0, 1, 0]
            [0, 0, 0, 1]
        ]
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert_eq!(
    ///     mat4![
    ///         [0, 9, 3, 0]
    ///         [9, 8, 0, 8]
    ///         [1, 8, 5, 3]
    ///         [0, 0, 5, 8]
    ///     ].transpose(),
    ///     mat4![
    ///         [0, 9, 1, 0]
    ///         [9, 8, 8, 0]
    ///         [3, 0, 5, 5]
    ///         [0, 8, 3, 8]
    ///     ]
    /// );
    ///
    /// // Transposing the identity matrix
    /// assert_eq!(
    ///     Mat4::identity().transpose(), Mat4::identity()
    /// );
    /// ```
    pub fn transpose(&self) -> Self {
        let mut mat = Mat4::zero();
        for row in 0..4 {
            for col in 0..4 {
                mat[(row, col)] = self[(col, row)]
            }
        }
        mat
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert_eq!(
    ///     mat4![
    ///         [-6, 1, 1, 6]
    ///         [-8, 5, 8, 6]
    ///         [-1, 0, 8, 2]
    ///         [-7, 1, -1, 1]
    ///     ].submatrix(2, 1),
    ///     mat3![
    ///         [-6, 1, 6]
    ///         [-8, 8, 6]
    ///         [-7, -1, 1]
    ///     ]
    /// );
    /// ```
    pub fn submatrix(&self, row: usize, col: usize) -> Mat3 {
        let mut elements = Vec::new();
        for r in 0..4 {
            for c in 0..4 {
                if r != row && c != col {
                    elements.push(self[(r, c)]);
                }
            }
        }
        Mat3::from(&elements[..])
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert!(is_equal(
    ///     mat4![
    ///         [-6, 1, 1, 6]
    ///         [-8, 1, 2, 6]
    ///         [-1, -5, 8, -4]
    ///         [-7, 2, 6, 4]
    ///     ].minor(0, 0),
    ///     -196.0
    /// ));
    /// ```
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert!(is_equal(
    ///     mat4![
    ///         [-6, 1, 1, 6]
    ///         [1, -8, 2, 6]
    ///         [-5, -1, 8, -4]
    ///         [2, -7, 6, 4]
    ///     ].cofactor(0, 1),
    ///     196.0
    /// ));
    /// ```
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        (-1_i8).pow((row + col).try_into().unwrap()) as f64 * self.minor(row, col)
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert!(is_equal(
    ///     mat4![
    ///         [-2, -8, 3, 5]
    ///         [-3, 1, 7, 3]
    ///         [1, 2, -9, 6]
    ///         [-6, 7, 7, -9]
    ///     ].determinant(),
    ///     -4071.0
    /// ));
    /// // Noninvertible matrix
    /// assert!(is_equal(
    ///     mat4![
    ///         [-4, 2, -2, -3]
    ///         [9, 6, 2, 6]
    ///         [0, -5, 1, -5]
    ///         [0, 0, 0, 0]
    ///     ].determinant(),
    ///     0.0
    /// ));
    /// ```
    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for col in 0..4 {
            det += self[(0, col)] * self.cofactor(0, col)
        }
        det
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert_eq!(
    ///     mat4![
    ///         [-5, 2, 6, -8]
    ///         [1, -5, 1, 8]
    ///         [7, 7, -6, -7]
    ///         [1, -3, 7, 4]
    ///     ].inverse(),
    ///     mat4![
    ///         [0.21805, 0.45113, 0.24060, -0.04511]
    ///         [-0.80827, -1.45677, -0.44361, 0.52068]
    ///         [-0.07895, -0.22368, -0.05263, 0.19737]
    ///         [-0.52256, -0.81391, -0.30075, 0.30639]
    ///     ]
    /// );
    /// assert_eq!(
    ///     mat4![
    ///         [8, -5, 9, 2]
    ///         [7, 5, 6, 1]
    ///         [-6, 0, 9, 6]
    ///         [-3, 0, -9, -4]
    ///     ].inverse(),
    ///     mat4![
    ///         [-0.15385, -0.15385, -0.28205, -0.53846]
    ///         [-0.07692, 0.12308, 0.02564, 0.03077]
    ///         [0.35897, 0.35897, 0.43590, 0.92308]
    ///         [-0.69231, -0.69231, -0.76923, -1.92308]
    ///     ]
    /// );
    /// assert_eq!(
    ///     mat4![
    ///         [9, 3, 0, 9]
    ///         [-5, -2, -6, -3]
    ///         [-4, 9, 6, 4]
    ///         [-7, 6, 6, 2]
    ///     ].inverse(),
    ///     mat4![
    ///         [-0.04074, -0.07778, 0.14444, -0.22222]
    ///         [-0.07778, 0.03333, 0.36667, -0.33333]
    ///         [-0.02901, -0.14630, -0.10926, 0.12963]
    ///         [0.17778, 0.06667, -0.26667, 0.33333]
    ///     ]
    /// );
    /// ```
    pub fn inverse(&self) -> Mat4 {
        let det = self.determinant();
        if is_equal(det, 0.0) {
            panic!("non-invertible matrix: determinant is 0.0");
        }
        let mut mat = Mat4::zero();
        for row in 0..4 {
            for col in 0..4 {
                // (col, row) here instead of (row, col),
                // accomplishes the transpose operation!
                mat[(col, row)] = self.cofactor(row, col) / det;
            }
        }
        mat
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert_eq!(
    ///     Mat4::identity().translate(5, -3, 2) * Tuple::from(point![-3, 4, 5]),
    ///    Tuple::from(point![2, 1, 7])
    /// );
    ///
    /// // Multipying by the inverse of a translation matrix
    /// assert_eq!(
    ///     Mat4::identity().translate(5, -3, 2).inverse() * Tuple::from(point![-3, 4, 5]),
    ///    Tuple::from(point![-8, 7, 3])
    /// );
    ///
    /// // Translation does not affect vectors
    /// assert_eq!(
    ///     Mat4::identity().translate(5, -3, 2) * Tuple::from(vector![-3, 4, 5]),
    ///    Tuple::from(vector![-3, 4, 5])
    /// );
    /// ```
    pub fn translate<T, U, V>(self, x: T, y: U, z: V) -> Self
    where
        f64: From<T>,
        f64: From<U>,
        f64: From<V>,
    {
        let mut transform = Mat4::identity();
        transform[(0, 3)] = f64::from(x);
        transform[(1, 3)] = f64::from(y);
        transform[(2, 3)] = f64::from(z);
        transform * self
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// // A scaling matrix applied to a point
    /// assert_eq!(
    ///     Mat4::identity().scale(2, 3, 4) * Tuple::from(point![-4, 6, 8]),
    ///     Tuple::from(point![-8, 18, 32])
    /// );
    ///
    /// // A scaling matrix applied to a vector
    /// assert_eq!(
    ///     Mat4::identity().scale(2, 3, 4) * Tuple::from(vector![-4, 6, 8]),
    ///     Tuple::from(vector![-8, 18, 32])
    /// );
    ///
    /// // Multiplying by the inverse of a scaling matrix
    /// assert_eq!(
    ///     Mat4::identity().scale(2, 3, 4).inverse() * Tuple::from(vector![-4, 6, 8]),
    ///     Tuple::from(vector![-2, 2, 2])
    /// );
    ///
    /// // Reflection is scaling by a negative value
    /// assert_eq!(
    ///     Mat4::identity().scale(-1, 1, 1) * Tuple::from(point![2, 3, 4]),
    ///     Tuple::from(point![-2, 3, 4])
    /// );
    /// ```
    pub fn scale<T, U, V>(self, x: T, y: U, z: V) -> Self
    where
        f64: From<T>,
        f64: From<U>,
        f64: From<V>,
    {
        let mut transform = Mat4::identity();
        transform[(0, 0)] = f64::from(x);
        transform[(1, 1)] = f64::from(y);
        transform[(2, 2)] = f64::from(z);
        transform * self
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// # use core::f64::consts::{PI, SQRT_2};
    /// assert_eq!(
    ///     Mat4::identity().rotate_x(PI / 4.0) * Tuple::from(point![0, 1, 0]),
    ///     Tuple::from(point![0, SQRT_2 / 2.0, SQRT_2 / 2.0])
    /// );
    /// assert_eq!(
    ///     Mat4::identity().rotate_x(PI / 2.0) * Tuple::from(point![0, 1, 0]),
    ///     Tuple::from(point![0, 0, 1])
    /// );
    ///
    /// // The inverse of an x-rotation rotates in the opposite direction
    /// assert_eq!(
    ///     Mat4::identity().rotate_x(PI / 4.0).inverse() * Tuple::from(point![0, 1, 0]),
    ///     Tuple::from(point![0, SQRT_2 / 2.0, -SQRT_2 / 2.0])
    /// );
    /// ```
    pub fn rotate_x(self, rad: f64) -> Self {
        let mut transform = Mat4::identity();
        transform[(1, 1)] = rad.cos();
        transform[(1, 2)] = -rad.sin();
        transform[(2, 1)] = rad.sin();
        transform[(2, 2)] = rad.cos();
        transform * self
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// # use core::f64::consts::{PI, SQRT_2};
    /// assert_eq!(
    ///     Mat4::identity().rotate_y(PI / 4.0) * Tuple::from(point![0, 0, 1]),
    ///     Tuple::from(point![SQRT_2 / 2.0, 0, SQRT_2 / 2.0])
    /// );
    /// assert_eq!(
    ///     Mat4::identity().rotate_y(PI / 2.0) * Tuple::from(point![0, 0, 1]),
    ///     Tuple::from(point![1, 0, 0])
    /// );
    /// ```
    pub fn rotate_y(self, rad: f64) -> Self {
        let mut transform = Mat4::identity();
        transform[(0, 0)] = rad.cos();
        transform[(0, 2)] = rad.sin();
        transform[(2, 0)] = -rad.sin();
        transform[(2, 2)] = rad.cos();
        transform * self
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// # use core::f64::consts::{PI, SQRT_2};
    /// assert_eq!(
    ///     Mat4::identity().rotate_z(PI / 4.0) * Tuple::from(point![0, 1, 0]),
    ///     Tuple::from(point![-SQRT_2 / 2.0, SQRT_2 / 2.0, 0])
    /// );
    /// assert_eq!(
    ///     Mat4::identity().rotate_z(PI / 2.0) * Tuple::from(point![0, 1, 0]),
    ///     Tuple::from(point![-1, 0, 0])
    /// );
    /// ```
    pub fn rotate_z(self, rad: f64) -> Self {
        let mut transform = Mat4::identity();
        transform[(0, 0)] = rad.cos();
        transform[(0, 1)] = -rad.sin();
        transform[(1, 0)] = rad.sin();
        transform[(1, 1)] = rad.cos();
        transform * self
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// // A shearing transformation moves x in proportion to y
    /// assert_eq!(
    ///     Mat4::identity().shear(1, 0, 0, 0, 0, 0) * Tuple::from(point![2, 3, 4]),
    ///     Tuple::from(point![5, 3 ,4])
    /// );
    ///
    /// // A shearing transformation moves x in proportion to z
    /// assert_eq!(
    ///     Mat4::identity().shear(0, 1, 0, 0, 0, 0) * Tuple::from(point![2, 3, 4]),
    ///     Tuple::from(point![6, 3 ,4])
    /// );
    ///
    /// // A shearing transformation moves y in proportion to x
    /// assert_eq!(
    ///     Mat4::identity().shear(0, 0, 1, 0, 0, 0) * Tuple::from(point![2, 3, 4]),
    ///     Tuple::from(point![2, 5, 4])
    /// );
    ///
    /// // A shearing transformation moves y in proportion to z
    /// assert_eq!(
    ///     Mat4::identity().shear(0, 0, 0, 1, 0, 0) * Tuple::from(point![2, 3, 4]),
    ///     Tuple::from(point![2, 7 ,4])
    /// );
    ///
    /// // A shearing transformation moves z in proportion to x
    /// assert_eq!(
    ///     Mat4::identity().shear(0, 0, 0, 0, 1, 0) * Tuple::from(point![2, 3, 4]),
    ///     Tuple::from(point![2, 3, 6])
    /// );
    ///
    /// // A shearing transformation moves z in proportion to y
    /// assert_eq!(
    ///     Mat4::identity().shear(0, 0, 0, 0, 0, 1) * Tuple::from(point![2, 3, 4]),
    ///     Tuple::from(point![2, 3 ,7])
    /// );
    /// ```
    pub fn shear<XY, XZ, YX, YZ, ZX, ZY>(
        self,
        x_y: XY,
        x_z: XZ,
        y_x: YX,
        y_z: YZ,
        z_x: ZX,
        z_y: ZY,
    ) -> Self
    where
        f64: From<XY>,
        f64: From<XZ>,
        f64: From<YX>,
        f64: From<YZ>,
        f64: From<ZX>,
        f64: From<ZY>,
    {
        let mut transform = Mat4::identity();
        transform[(0, 1)] = f64::from(x_y);
        transform[(0, 2)] = f64::from(x_z);
        transform[(1, 0)] = f64::from(y_x);
        transform[(1, 2)] = f64::from(y_z);
        transform[(2, 0)] = f64::from(z_x);
        transform[(2, 1)] = f64::from(z_y);
        transform * self
    }
}

impl PartialEq for Mat4 {
    fn eq(&self, rhs: &Self) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if !is_equal(self[(row, col)], rhs[(row, col)]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Clone for Mat4 {
    fn clone(&self) -> Self {
        Self {
            elements: self.elements.clone(),
        }
    }
}

impl From<&[f64]> for Mat4 {
    fn from(elements: &[f64]) -> Self {
        if elements.len() != 16 {
            panic!("incompatible size for Mat4, size is {}", elements.len());
        }
        Self {
            elements: Vec::from(elements),
        }
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// let mat = mat4![
///     [1, 2, 3, 4]
///     [5.5, 6.5, 7.5, 8.5]
///     [9, 10, 11, 12]
///     [13.5, 14.5, 15.5, 16.5]
/// ];
/// assert!(is_equal(mat[(0, 0)], 1.0));
/// assert!(is_equal(mat[(1, 2)], 7.5));
/// assert!(is_equal(mat[(2, 2)], 11.0));
/// assert!(is_equal(mat[(3, 2)], 15.5));
/// ```
impl Index<(usize, usize)> for Mat4 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= 4 || index.1 >= 4 {
            panic!(
                "index out of bounds: Mat4 is 4 by 4, index is [({}, {})]",
                index.0, index.1
            );
        }
        &self.elements[index.0 * 4 + index.1]
    }
}

impl IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= 4 || index.1 >= 4 {
            panic!(
                "index out of bounds: Mat4 is 4 by 4, index is [({}, {})]",
                index.0, index.1
            );
        }
        &mut self.elements[index.0 * 4 + index.1]
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// let mat_a = mat4![
///     [1, 2, 3, 4]
///     [5, 6, 7, 8]
///     [9, 8, 7, 6]
///     [5, 4, 3, 2]
/// ];
/// let mat_b = mat4![
///     [-2, 1, 2, 3]
///     [3, 2, 1, -1]
///     [4, 3, 6, 5]
///     [1, 2, 7, 8]
/// ];
/// assert_eq!(
///     mat_a * mat_b,
///     mat4![
///         [20, 22, 50, 48]
///         [44, 54, 114, 108]
///         [40, 58, 110, 102]
///         [16, 26, 46, 42]
///     ]
/// );
/// // Multiplying a product by its inverse
/// let mat_a = mat4![
///     [3, -9, 7, 3]
///     [3, -8, 2, -9]
///     [-4, 4, 4, 1]
///     [-6, 5, -1, 1]
/// ];
/// let mat_b = mat4![
///     [8, 2, 2, 2]
///     [3, -1, 7, 0]
///     [7, 0, 5, 4]
///     [6, -2, 0, 5]
/// ];
/// let mat_c = mat_a.clone() * mat_b.clone();
/// assert_eq!(
///     mat_c * mat_b.inverse(),
///     mat_a
/// );
/// ```
impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut mat = Mat4::zero();
        for row in 0..4 {
            for col in 0..4 {
                mat[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)]
                    + self[(row, 3)] * rhs[(3, col)]
            }
        }
        mat
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// let mat = mat4![
///     [1, 2, 3, 4]
///     [2, 4, 4, 2]
///     [8, 6, 4, 1]
///     [0, 0, 0, 1]
/// ];
/// let tuple = Tuple::from(point![1, 2, 3]);
/// assert_eq!(mat * tuple, tuple![18, 24, 33, 1]);
/// ```
impl Mul<Tuple> for Mat4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self[(0, 0)] * rhs.x
                + self[(0, 1)] * rhs.y
                + self[(0, 2)] * rhs.z
                + self[(0, 3)] * rhs.w,
            y: self[(1, 0)] * rhs.x
                + self[(1, 1)] * rhs.y
                + self[(1, 2)] * rhs.z
                + self[(1, 3)] * rhs.w,
            z: self[(2, 0)] * rhs.x
                + self[(2, 1)] * rhs.y
                + self[(2, 2)] * rhs.z
                + self[(2, 3)] * rhs.w,
            w: self[(3, 0)] * rhs.x
                + self[(3, 1)] * rhs.y
                + self[(3, 2)] * rhs.z
                + self[(3, 3)] * rhs.w,
        }
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// let mat = mat4![
///     [1, 2, 3, 4]
///     [2, 4, 4, 2]
///     [8, 6, 4, 1]
///     [0, 0, 0, 1]
/// ];
/// let point = point![1, 2, 3];
/// assert_eq!(mat * point, point![18, 24, 33]);
/// ```
impl Mul<Point> for Mat4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let rhs = Tuple::from(rhs);
        Point {
            x: self[(0, 0)] * rhs.x
                + self[(0, 1)] * rhs.y
                + self[(0, 2)] * rhs.z
                + self[(0, 3)] * rhs.w,
            y: self[(1, 0)] * rhs.x
                + self[(1, 1)] * rhs.y
                + self[(1, 2)] * rhs.z
                + self[(1, 3)] * rhs.w,
            z: self[(2, 0)] * rhs.x
                + self[(2, 1)] * rhs.y
                + self[(2, 2)] * rhs.z
                + self[(2, 3)] * rhs.w,
        }
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// let mat = mat4![
///     [1, 2, 3, 4]
///     [2, 4, 4, 2]
///     [8, 6, 4, 1]
///     [0, 0, 0, 1]
/// ];
/// let vector = vector![1, 2, 3];
/// assert_eq!(mat * vector, vector![14, 22, 32]);
/// ```
impl Mul<Vector> for Mat4 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let rhs = Tuple::from(rhs);
        Vector {
            x: self[(0, 0)] * rhs.x
                + self[(0, 1)] * rhs.y
                + self[(0, 2)] * rhs.z
                + self[(0, 3)] * rhs.w,
            y: self[(1, 0)] * rhs.x
                + self[(1, 1)] * rhs.y
                + self[(1, 2)] * rhs.z
                + self[(1, 3)] * rhs.w,
            z: self[(2, 0)] * rhs.x
                + self[(2, 1)] * rhs.y
                + self[(2, 2)] * rhs.z
                + self[(2, 3)] * rhs.w,
        }
    }
}

/// 3 by 3 matrix
///
/// # Examples
///
/// ```
/// # use trace::prelude::*;
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

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert_eq!(
    ///     mat3![
    ///         [1, 5, 0]
    ///         [-3, 2, 7]
    ///         [0, 6, -3]
    ///     ].submatrix(0, 2),
    ///     mat2![
    ///         [-3, 2]
    ///         [0, 6]
    ///     ]
    /// );
    /// ```
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

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert!(is_equal(
    ///     mat3![
    ///         [3, 5, 0]
    ///         [2, -1, -7]
    ///         [6, -1, 5]
    ///     ].minor(1, 0),
    ///     25.0
    /// ));
    /// ```
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// let mat = mat3![
    ///     [3, 5, 0]
    ///     [2, -1, -7]
    ///     [6, -1, 5]
    /// ];
    /// assert!(is_equal(
    ///     mat.cofactor(0, 0),
    ///     -12.0
    /// ));
    /// assert!(is_equal(
    ///     mat.cofactor(1, 0),
    ///     -25.0
    /// ));
    /// ```
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        (-1_i8).pow((row + col).try_into().unwrap()) as f64 * self.minor(row, col)
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert!(is_equal(
    ///     mat3![
    ///         [1, 2, 6]
    ///         [-5, 8, -4]
    ///         [2, 6, 4]
    ///     ].determinant(),
    ///     -196.0
    /// ));
    /// ```
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

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// let mat = mat3![
///     [-3, 5, 0]
///     [1, -2, -7]
///     [0, 1, 1]
/// ];
/// assert!(is_equal(mat[(0, 0)], -3.0));
/// assert!(is_equal(mat[(1, 1)], -2.0));
/// assert!(is_equal(mat[(2, 2)], 1.0));
/// ```
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

/// 2 by 2 matrix
///
/// # Examples
///
/// ```
/// # use trace::prelude::*;
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

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert!(is_equal(
    ///     mat2![
    ///         [1, 5]
    ///         [-3, 2]
    ///     ].determinant(),
    ///     17.0
    /// ));
    /// ```
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

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// let mat = mat2![
///     [-3, 5]
///     [1, -2]
/// ];
/// assert!(is_equal(mat[(0, 0)], -3.0));
/// assert!(is_equal(mat[(0, 1)], 5.0));
/// assert!(is_equal(mat[(1, 0)], 1.0));
/// assert!(is_equal(mat[(1, 1)], -2.0));
/// ```
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
    use crate::point;
    use crate::point::Point;
    use core::f64::consts::PI;

    #[test]
    fn test_identity_inverse() {
        let identity = Mat4::identity();
        assert_eq!(identity.inverse(), identity);
    }

    #[test]
    fn test_mul_by_inverse() {
        let mat = mat4![
            [8, 2, 2, 2]
            [3, -1, 7, 0]
            [7, 0, 5, 4]
            [6, -2, 0, 5]
        ];
        assert_eq!(mat.clone() * mat.inverse(), Mat4::identity());
    }

    #[test]
    fn test_inverse_transpose() {
        let mat = mat4![
            [8, 2, 2, 2]
            [3, -1, 7, 0]
            [7, 0, 5, 4]
            [6, -2, 0, 5]
        ];
        // Inverse of the transpose of a matrix, is equal to
        // the transpose of the inverse of the same matrix.
        assert_eq!(mat.transpose().inverse(), mat.inverse().transpose());
    }

    #[test]
    fn test_chain_transform() {
        assert_eq!(
            Mat4::identity()
                .rotate_x(PI / 2.0)
                .scale(5, 5, 5)
                .translate(10, 5, 7)
                * Tuple::from(point![1, 0, 1]),
            Tuple::from(point![15, 0, 7])
        );
    }
}
