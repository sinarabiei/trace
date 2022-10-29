use crate::prelude::{is_equal, Vector};
use std::ops::{Add, Sub};

/// Point in 3D space
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Creates a `Point` containing the arguments.
///
/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(point![1.0, 2, 3.05], Point {x: 1.0, y: 2.0, z: 3.05})
/// ```
#[macro_export]
macro_rules! point {
    [$x: expr, $y: expr, $z: expr]=>{
	{
	    Point {
		x: f64::from($x),
		y: f64::from($y),
		z: f64::from($z),
	    }
	}
    }
}

impl Point {
    /// Creates a Point with all elements equal to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert_eq!(Point::zero(), point![0, 0, 0]);
    /// ```
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(point![1, 2, 3], Point {x: 1.0, y: 2.0, z: 3.0});
/// ```
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        is_equal(self.x, other.x) && is_equal(self.y, other.y) && is_equal(self.z, other.z)
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(point![1, 2.4, 5.0] + vector![6.0, 9.01, 0.7], point![7, 11.41, 5.7]);
/// ```
impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(point![3, 2, 1] - point![5, 6, 7], vector![-2, -4, -6]);
/// ```
impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(point![3, 2, 1] - vector![5, 6, 7], point![-2, -4, -6]);
/// ```
impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {}
