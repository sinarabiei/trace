use super::Pattern;
use crate::color::Color;
use crate::mat4::Mat4;
use crate::pattern::solid::Solid;
use crate::point::Point;

/// Interpolates between two colors radially.
#[derive(Debug)]
pub struct RadialGradient {
    a: Color,
    b: Color,
    transform: Mat4,
}

impl RadialGradient {
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

impl Default for RadialGradient {
    fn default() -> Self {
        Self {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: Mat4::identity(),
        }
    }
}

impl Pattern for RadialGradient {
    fn at(&self, point: Point) -> Color {
        let distance = self.b - self.a;
        let fraction = ((point.x.powf(2.0) + point.z.powf(2.0)).sqrt())
            - ((point.x.powf(2.0) + point.z.powf(2.0)).sqrt().floor());
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
pub struct RadialGradientNested {
    a: Box<dyn Pattern>,
    b: Box<dyn Pattern>,
    transform: Mat4,
}

impl RadialGradientNested {
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

impl Pattern for RadialGradientNested {
    fn at(&self, point: Point) -> Color {
        let distance = self.b.at(point) - self.a.at(point);
        let fraction = ((point.x.powf(2.0) + point.z.powf(2.0)).sqrt())
            - ((point.x.powf(2.0) + point.z.powf(2.0)).sqrt().floor());
        self.a.at(point) + distance * fraction
    }

    fn transform(&self) -> &Mat4 {
        &self.transform
    }

    fn debug_local(&self) -> String {
        format!("{:?}", self)
    }
}

impl Default for RadialGradientNested {
    fn default() -> Self {
        Self {
            a: Box::new(Solid::new(Color::WHITE)),
            b: Box::new(Solid::new(Color::BLACK)),
            transform: Mat4::identity(),
        }
    }
}
