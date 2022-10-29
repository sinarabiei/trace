use super::Pattern;
use crate::color::Color;
use crate::mat4::Mat4;
use crate::pattern::solid::Solid;
use crate::point::Point;

#[derive(Debug)]
pub struct Gradient {
    a: Color,
    b: Color,
    transform: Mat4,
}

impl Gradient {
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

impl Default for Gradient {
    fn default() -> Self {
        Self {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: Mat4::identity(),
        }
    }
}

impl Pattern for Gradient {
    fn at(&self, point: Point) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();
        self.a + distance * fraction
    }

    fn transform(&self) -> &Mat4 {
        &self.transform
    }

    fn debug_local(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
pub struct GradientNested {
    a: Box<dyn Pattern>,
    b: Box<dyn Pattern>,
    transform: Mat4,
}

impl GradientNested {
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

impl Pattern for GradientNested {
    fn at(&self, point: Point) -> Color {
        let distance = self.b.at(point) - self.a.at(point);
        let fraction = point.x - point.x.floor();
        self.a.at(point) + distance * fraction
    }

    fn transform(&self) -> &Mat4 {
        &self.transform
    }

    fn debug_local(&self) -> String {
        format!("{:?}", self)
    }
}

impl Default for GradientNested {
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
    use crate::color;
    use crate::point;

    #[test]
    fn test_at() {
        // A gradient linearly interpolates between colors
        let pattern = Gradient::default();
        assert_eq!(pattern.at(point![0, 0, 0]), Color::WHITE);
        assert_eq!(pattern.at(point![0.25, 0, 0]), color![0.75, 0.75, 0.75]);
        assert_eq!(pattern.at(point![0.5, 0, 0]), color![0.5, 0.5, 0.5]);
        assert_eq!(pattern.at(point![0.75, 0, 0]), color![0.25, 0.25, 0.25]);
    }
}
