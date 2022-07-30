use crate::point::Point;
use crate::vector::Vector;

/// Tuple
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[macro_export]
macro_rules! tuple {
    [$x: expr, $y: expr, $z: expr, $w: expr]=>{
	{
	    Tuple {
		x: f64::from($x),
		y: f64::from($y),
		z: f64::from($z),
		w: f64::from($w),
	    }
	}
    }
}

impl Tuple {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl From<Point> for Tuple {
    fn from(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
            z: point.z,
            w: 1.0,
        }
    }
}

impl From<Vector> for Tuple {
    fn from(vector: Vector) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            w: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {}
