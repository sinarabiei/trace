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

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Converts vector into a unit vector.
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        if is_equal(magnitude, 0.0) {
            eprintln!(
                "\x1b[1;33mwarning\x1b[0m: {}\n",
                "normalizing zero magnitude vector"
            );
            return self.clone();
        }
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    /// Dot product of two vectors.
    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Cross product of two vectors.
    pub fn cross(&self, rhs: Self) -> Self {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn reflect(self, rhs: Self) -> Self {
        self - rhs * 2 * self.dot(rhs)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        is_equal(self.x, other.x) && is_equal(self.y, other.y) && is_equal(self.z, other.z)
    }
}

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
mod tests {
    use super::*;
    use std::f64::consts::SQRT_2;

    #[test]
    fn test_magnitude() {
        assert!(is_equal(vector![1, 0, 0].magnitude(), 1.0));
        assert!(is_equal(vector![0, 1, 0].magnitude(), 1.0));
        assert!(is_equal(vector![0, 0, 1].magnitude(), 1.0));
        assert!(is_equal(vector![1, 2, 3].magnitude(), 14.0_f64.sqrt()));
        assert!(is_equal(vector![-1, -2, -3].magnitude(), 14.0_f64.sqrt()));
    }

    #[test]
    fn test_normalize() {
        assert_eq!(vector![4, 0, 0].normalize(), vector![1, 0, 0]);
        assert_eq!(
            vector![1, 2, 3].normalize(),
            vector![0.26726, 0.53452, 0.80178]
        );

        // Magnitude of a normalized vector is 1.0
        assert!(is_equal(vector![1, 2, 3].normalize().magnitude(), 1.0));

        // Normalizing a zero magnitude vector is the zero vector
        assert_eq!(vector![0, 0, 0].normalize(), vector![0, 0, 0]);
    }

    #[test]
    fn test_dot() {
        assert!(is_equal(vector![1, 2, 3].dot(vector![2, 3, 4]), 20.0));
    }

    #[test]
    fn test_cross() {
        assert_eq!(vector![1, 2, 3].cross(vector![2, 3, 4]), vector![-1, 2, -1]);
        assert_eq!(vector![2, 3, 4].cross(vector![1, 2, 3]), vector![1, -2, 1]);
    }

    #[test]
    fn test_reflect() {
        // Reflecting a vector approaching at 45 degrees
        let vector = vector![1, -1, 0];
        let normal = vector![0, 1, 0];
        assert_eq!(vector.reflect(normal), vector![1, 1, 0]);

        // Reflecting a vector off a slanted surface
        let vector = vector![0, -1, 0];
        let normal = vector![SQRT_2 / 2.0, SQRT_2 / 2.0, 0];
        assert_eq!(vector.reflect(normal), vector![1, 0, 0]);
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            vector![1, 2, 3],
            Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            vector![1, 2.4, 5.0] + vector![6.0, 9.01, 0.7],
            vector![7, 11.41, 5.7]
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(vector![3, 2, 1] - vector![5, 6, 7], vector![-2, -4, -6]);
    }

    #[test]
    fn test_neg() {
        assert_eq!(-vector![1, -2, 3], vector![-1, 2, -3]);
    }

    #[test]
    fn test_mul() {
        assert_eq!(vector![1, -2, 3] * 3.5, vector![3.5, -7, 10.5]);
        assert_eq!(vector![1, -2, 3] * 0.5, vector![0.5, -1, 1.5]);
    }

    #[test]
    fn test_div() {
        assert_eq!(vector![1, -2, 3] / 2.0, vector![0.5, -1, 1.5]);
    }
}
