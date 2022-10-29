use super::Pattern;
use crate::color::Color;
use crate::mat4::Mat4;
use crate::pattern::solid::Solid;
use crate::point::Point;
use crate::prelude::is_equal;

#[derive(Debug)]
pub struct Stripe {
    a: Color,
    b: Color,
    transform: Mat4,
}

impl Stripe {
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

impl Pattern for Stripe {
    fn at(&self, point: Point) -> Color {
        if is_equal(point.x.floor() % 2.0, 0.0) {
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

impl Default for Stripe {
    fn default() -> Self {
        Self {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: Mat4::identity(),
        }
    }
}

#[derive(Debug)]
pub struct StripeNested {
    a: Box<dyn Pattern>,
    b: Box<dyn Pattern>,
    transform: Mat4,
}

impl StripeNested {
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

impl Pattern for StripeNested {
    fn at(&self, point: Point) -> Color {
        if is_equal(point.x.floor() % 2.0, 0.0) {
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

impl Default for StripeNested {
    fn default() -> Self {
        Self {
            a: Box::new(Solid::new(Color::WHITE)),
            b: Box::new(Solid::new(Color::BLACK)),
            transform: Mat4::identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point;

    #[test]
    fn test_at() {
        // A stripe pattern is constant in y
        let pattern = Stripe::default();
        assert_eq!(pattern.at(point!(0, 0, 0)), Color::WHITE);
        assert_eq!(pattern.at(point!(0, 1, 0)), Color::WHITE);
        assert_eq!(pattern.at(point!(0, 2, 0)), Color::WHITE);

        // A stripe pattern is constant in z
        let pattern = Stripe::default();
        assert_eq!(pattern.at(point!(0, 0, 0)), Color::WHITE);
        assert_eq!(pattern.at(point!(0, 0, 1)), Color::WHITE);
        assert_eq!(pattern.at(point!(0, 0, 2)), Color::WHITE);

        // A stripe pattern alternates in x
        let pattern = Stripe::default();
        assert_eq!(pattern.at(point!(0, 0, 0)), Color::WHITE);
        assert_eq!(pattern.at(point!(0.9, 0, 0)), Color::WHITE);
        assert_eq!(pattern.at(point!(1, 0, 0)), Color::BLACK);
        assert_eq!(pattern.at(point!(-0.1, 0, 0)), Color::BLACK);
        assert_eq!(pattern.at(point!(-1, 0, 0)), Color::BLACK);
        assert_eq!(pattern.at(point!(-1.1, 0, 0)), Color::WHITE);
    }
}
