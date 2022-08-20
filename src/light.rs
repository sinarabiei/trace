use crate::color::Color;
use crate::point::Point;

#[derive(Copy, Clone)]
pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

impl Light {}
