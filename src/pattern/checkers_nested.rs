use super::Pattern;
use crate::color::Color;
use crate::mat4::Mat4;
use crate::point::Point;
use crate::prelude::is_equal;

#[derive(Debug)]
pub struct CheckersNested {
    a: Box<dyn Pattern>,
    b: Box<dyn Pattern>,
    transform: Mat4,
}

impl CheckersNested {
    pub fn new<T, U>(a: T, b: U) -> Self
    where
        T: Pattern + 'static,
        U: Pattern + 'static,
    {
        Self {
            a: Box::new(a),
            b: Box::new(b),
            transform: Mat4::identity(),
        }
    }

    pub fn set_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;

        self
    }
}

impl Pattern for CheckersNested {
    fn at(&self, point: Point) -> Color {
        if is_equal(
            (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0,
            0.0,
        ) {
            self.a.at(point)
        } else {
            self.b.at(point)
        }
    }

    fn transform(&self) -> &Mat4 {
        &self.transform
    }

    fn debug_local(&self) -> String {
        format!("{:?}", self)
    }
}
