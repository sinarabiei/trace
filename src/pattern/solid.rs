use super::Pattern;
use crate::color::Color;
use crate::mat4::Mat4;
use crate::point::Point;

#[derive(Debug)]
pub struct Solid {
    a: Color,
    transform: Mat4,
}

impl Solid {
    pub fn new(a: Color) -> Self {
        Self {
            a,
            transform: Mat4::identity(),
        }
    }

    pub fn set_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;

        self
    }
}

impl Default for Solid {
    fn default() -> Self {
        Self {
            a: Color::WHITE,
            transform: Mat4::identity(),
        }
    }
}

impl Pattern for Solid {
    fn at(&self, _point: Point) -> Color {
        self.a
    }

    fn transform(&self) -> &Mat4 {
        &self.transform
    }

    fn debug_local(&self) -> String {
        format!("{:?}", self)
    }
}
