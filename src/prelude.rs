pub use crate::camera::Camera;
pub use crate::color;
pub use crate::color::Color;
pub use crate::light::Light;
pub use crate::mat4::Mat4;
pub use crate::material::Material;
pub use crate::pattern::{
    blended::Blended,
    checkers::Checkers,
    checkers_nested::CheckersNested,
    gradient::{Gradient, GradientNested},
    perturb::Perturb,
    radial_gradient::{RadialGradient, RadialGradientNested},
    ring::{Ring, RingNested},
    solid::Solid,
    stripe::{Stripe, StripeNested},
    Pattern,
};
pub use crate::plane::Plane;
pub use crate::point;
pub use crate::point::Point;
pub use crate::sphere::Sphere;
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
