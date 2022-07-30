use crate::prelude::is_equal;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Vector in 3D space
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Creates a `Vector` containing the arguments.
///
/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(vector![1.0, 2, 3.05], Vector {x: 1.0, y: 2.0, z: 3.05})
/// ```
#[macro_export]
macro_rules! vector {
    [$x: expr, $y: expr, $z: expr]=>{
	{
	    Vector {
		x: f64::from($x),
		y: f64::from($y),
		z: f64::from($z),
	    }
	}
    }
}

impl Vector {
    /// Creates a Vector with all elemnets equal to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert_eq!(Vector::zero(), vector![0, 0, 0]);
    /// ```
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vector {
    ///
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert!(is_equal(vector![1, 0, 0].magnitude(), 1.0));
    /// assert!(is_equal(vector![0, 1, 0].magnitude(), 1.0));
    /// assert!(is_equal(vector![0, 0, 1].magnitude(), 1.0));
    /// assert!(is_equal(vector![1, 2, 3].magnitude(), 14.0_f64.sqrt()));
    /// assert!(is_equal(vector![-1, -2, -3].magnitude(), 14.0_f64.sqrt()));
    /// ```
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Converts vector into a unit vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert_eq!(vector![4, 0, 0].normalize(), vector![1, 0, 0]);
    /// assert_eq!(vector![1, 2, 3].normalize(), vector![0.26726, 0.53452, 0.80178]);
    ///
    /// // Magnitude of a normalized vector is 1.0
    /// assert!(is_equal(vector![1, 2, 3].normalize().magnitude(), 1.0));
    /// ```
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    /// Dot product of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert!(is_equal(vector![1, 2, 3].dot(vector![2, 3, 4]), 20.0));
    /// ```
    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Cross product of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// assert_eq!(vector![1, 2, 3].cross(vector![2, 3, 4]), vector![-1, 2, -1]);
    /// assert_eq!(vector![2, 3, 4].cross(vector![1, 2, 3]), vector![1, -2, 1]);
    /// ```
    pub fn cross(&self, rhs: Self) -> Self {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(vector![1, 2, 3], Vector {x: 1.0, y: 2.0, z: 3.0});
/// ```
impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        is_equal(self.x, other.x) && is_equal(self.y, other.y) && is_equal(self.z, other.z)
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(vector![1, 2.4, 5.0] + vector![6.0, 9.01, 0.7], vector![7, 11.41, 5.7]);
/// ```
impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
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
/// assert_eq!(vector![3, 2, 1] - vector![5, 6, 7], vector![-2, -4, -6]);
/// ```
impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
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
/// assert_eq!(-vector![1, -2, 3], vector![-1, 2, -3]);
/// ```
impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(vector![1, -2, 3] * 3.5, vector![3.5, -7, 10.5]);
/// assert_eq!(vector![1, -2, 3] * 0.5, vector![0.5, -1, 1.5]);
/// ```
impl<T> Mul<T> for Vector
where
    f64: From<T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let scalar = f64::from(scalar);
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(vector![1, -2, 3] / 2.0, vector![0.5, -1, 1.5]);
/// ```
impl<T> Div<T> for Vector
where
    f64: From<T>,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        let scalar = f64::from(scalar);
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]
mod tests {}
