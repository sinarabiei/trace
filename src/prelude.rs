pub use crate::camera::Camera;
pub use crate::canvas::Canvas;
pub use crate::color;
pub use crate::color::Color;
pub use crate::intersection::Intersection;
pub use crate::light::Light;
pub use crate::mat2;
pub use crate::mat3;
pub use crate::mat4;
pub use crate::material::Material;
pub use crate::matrix::Mat2;
pub use crate::matrix::Mat3;
pub use crate::matrix::Mat4;
pub use crate::plane::Plane;
pub use crate::point;
pub use crate::point::Point;
pub use crate::ray::Ray;
pub use crate::shape::Shape;
pub use crate::sphere::Sphere;
pub use crate::tuple;
pub use crate::tuple::Tuple;
pub use crate::vector;
pub use crate::vector::Vector;
pub use crate::world::World;
use std::sync::atomic::AtomicUsize;

pub static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub const EPSILON: f64 = 0.00001;

/// Float numbers comparison.
pub fn is_equal(lhs: f64, rhs: f64) -> bool {
    (lhs - rhs).abs() < EPSILON
}
