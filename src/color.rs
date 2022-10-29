use crate::prelude::is_equal;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    pub const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
}

/// Creates a Color containing the arguments.
///
/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// assert_eq!(
///     color![-0.5, 0.4, 1.7],
///     Color {red: -0.5, green: 0.4, blue: 1.7}
/// );
/// ```
#[macro_export]
macro_rules! color {
    [$red: expr, $green: expr, $blue: expr]=>{
	{
	    Color {
		red: f64::from($red),
		green: f64::from($green),
		blue: f64::from($blue),
	    }
	}
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        is_equal(self.red, other.red)
            && is_equal(self.green, other.green)
            && is_equal(self.blue, other.blue)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

/// Hadamard product
impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl<T> Mul<T> for Color
where
    f64: From<T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let scalar = f64::from(scalar);
        Self {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            color![0.9, 0.6, 0.75] + color![0.7, 0.1, 0.25],
            color![1.6, 0.7, 1.0]
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            color![0.9, 0.6, 0.75] - color![0.7, 0.1, 0.25],
            color![0.2, 0.5, 0.5]
        );
    }

    #[test]
    fn test_mul_hadamard() {
        assert_eq!(
            color![1, 0.2, 0.4] * color![0.9, 1, 0.1],
            color![0.9, 0.2, 0.04]
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(color![0.2, 0.3, 0.4] * 2, color![0.4, 0.6, 0.8]);
    }
}
