use crate::color::Color;
use crate::mat4::Mat4;
use crate::point::Point;
use crate::shape::Shape;
use std::fmt::Debug;

pub mod blended;
pub mod checkers;
pub mod checkers_nested;
pub mod gradient;
pub mod perturb;
pub mod radial_gradient;
pub mod ring;
pub mod solid;
pub mod stripe;

pub trait Pattern {
    fn at_object(&self, object: &dyn Shape, world_point: Point) -> Color {
        let object_point = object.transform().inverse() * world_point;
        let pattern_point = self.transform().inverse() * object_point;
        self.at(pattern_point)
    }

    fn at(&self, point: Point) -> Color;
    fn transform(&self) -> &Mat4;

    fn debug_local(&self) -> String;
}

impl Debug for dyn Pattern {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self.debug_local())
    }
}

#[derive(Debug)]
pub struct TestPattern {
    pub transform: Mat4,
}

use crate::color;
impl Pattern for TestPattern {
    fn at(&self, point: Point) -> Color {
        color![point.x, point.y, point.z]
    }

    fn transform(&self) -> &Mat4 {
        &self.transform
    }

    fn debug_local(&self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;
    use crate::{color, point};

    #[test]
    fn test_at_object() {
        // A pattern with an object transformation
        let mut shape = Sphere::new();
        shape.transform = Mat4::identity().scale(2, 2, 2);
        let pattern = TestPattern {
            transform: Mat4::identity(),
        };
        println!("pattern.transform() is {:?}", pattern.transform());
        assert_eq!(
            pattern.at_object(&shape, point![2, 3, 4]),
            color![1, 1.5, 2]
        );

        // A pattern with a pattern transformation
        let shape = Sphere::new();
        let mut pattern = TestPattern {
            transform: Mat4::identity(),
        };
        pattern.transform = Mat4::identity().scale(2, 2, 2);
        assert_eq!(
            pattern.at_object(&shape, point![2, 3, 4]),
            color![1, 1.5, 2]
        );

        // A pattern with both an object and a pattern transformation
        let mut shape = Sphere::new();
        shape.transform = Mat4::identity().scale(2, 2, 2);
        let mut pattern = TestPattern {
            transform: Mat4::identity(),
        };
        pattern.transform = Mat4::identity().translate(0.5, 1, 1.5);
        assert_eq!(
            pattern.at_object(&shape, point![2.5, 3, 3.5]),
            color![0.75, 0.5, 0.25]
        );
    }
}
