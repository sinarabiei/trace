use super::Pattern;
use crate::color::Color;
use crate::mat4::Mat4;
use crate::point::Point;
use crate::prelude::is_equal;

#[derive(Debug)]
pub struct Checkers {
    a: Color,
    b: Color,
    transform: Mat4,
}

impl Checkers {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Mat4::identity(),
        }
    }

    pub fn set_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;

        self
    }
}

impl Default for Checkers {
    fn default() -> Self {
        Self {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: Mat4::identity(),
        }
    }
}

impl Pattern for Checkers {
    fn at(&self, point: Point) -> Color {
        if is_equal(
            (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0,
            0.0,
        ) {
            self.a
        } else {
            self.b
        }
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
    use crate::point;

    #[test]
    fn test_at() {
        // Checkers should repeat in x
        let pattern = Checkers::default();
        assert_eq!(pattern.at(point![0, 0, 0]), Color::WHITE);
        assert_eq!(pattern.at(point![0.99, 0, 0]), Color::WHITE);
        assert_eq!(pattern.at(point![1.01, 0, 0]), Color::BLACK);

        // Checkers should repeat in y
        let pattern = Checkers::default();
        assert_eq!(pattern.at(point![0, 0, 0]), Color::WHITE);
        assert_eq!(pattern.at(point![0, 0.99, 0]), Color::WHITE);
        assert_eq!(pattern.at(point![0, 1.01, 0]), Color::BLACK);

        // Checkers should repeat in z
        let pattern = Checkers::default();
        assert_eq!(pattern.at(point![0, 0, 0]), Color::WHITE);
        assert_eq!(pattern.at(point![0, 0, 0.99]), Color::WHITE);
        assert_eq!(pattern.at(point![0, 0, 1.01]), Color::BLACK);
    }
}
