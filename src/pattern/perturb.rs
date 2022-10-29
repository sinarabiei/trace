use super::Pattern;
use crate::color::Color;
use crate::mat4::Mat4;
use crate::point::Point;
use noise::{NoiseFn, Perlin};

#[derive(Debug)]
pub struct Perturb {
    pattern: Box<dyn Pattern>,
    transform: Mat4,
    perlin: Perlin,
}

impl Perturb {
    pub fn new<T>(pattern: T) -> Self
    where
        T: Pattern + 'static,
    {
        Self {
            pattern: Box::new(pattern),
            transform: Mat4::identity(),
            perlin: Perlin::default(),
        }
    }

    pub fn set_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;

        self
    }
}

impl Pattern for Perturb {
    fn at(&self, point: Point) -> Color {
        let noise = self.perlin.get([point.x, point.y, point.z]);
        let point = Point {
            x: point.x + noise * 0.2,
            y: point.y + noise * 0.2,
            z: point.z + noise * 0.2,
        };

        self.pattern.at(point)
    }

    fn transform(&self) -> &Mat4 {
        &self.transform
    }

    fn debug_local(&self) -> String {
        format!("{:?}", self)
    }
}
