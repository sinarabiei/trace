use super::Pattern;
use crate::color::Color;
use crate::mat4::Mat4;
use crate::point::Point;

#[derive(Debug)]
pub struct Blended {
    a: Box<dyn Pattern>,
    b: Box<dyn Pattern>,
    transform: Mat4,
}

impl Blended {
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

impl Pattern for Blended {
    fn at(&self, point: Point) -> Color {
        let color_a = self.a.at(point);
        let color_b = self.b.at(point);
        (color_a + color_b) * 0.5
    }

    fn transform(&self) -> &Mat4 {
        &self.transform
    }

    fn debug_local(&self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(test)]
mod tests {}
