pub use crate::canvas::Canvas;
pub use crate::color;
pub use crate::color::Color;
pub use crate::point;
pub use crate::point::Point;
pub use crate::vector;
pub use crate::vector::Vector;

pub const EPSILON: f64 = 0.00001;

/// Float numbers comparison.
pub fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
