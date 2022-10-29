use crate::matrix::Mat4;
use crate::point::Point;
use crate::vector::Vector;

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// let ray = Ray {
///     origin: point![1, 2, 3],
///     direction: vector![4, 5, 6],
/// };
/// assert_eq!(ray.origin, point![1, 2, 3]);
/// assert_eq!(ray.direction, vector![4, 5, 6]);
/// ```
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// let ray = Ray {
    ///     origin: point![2, 3, 4],
    ///     direction: vector![1, 0, 0],
    /// };
    /// assert_eq!(ray.position(0), point![2, 3, 4]);
    /// assert_eq!(ray.position(1), point![3, 3, 4]);
    /// assert_eq!(ray.position(-1), point![1, 3, 4]);
    /// assert_eq!(ray.position(2.5), point![4.5, 3, 4]);
    /// ```
    pub fn position<T>(&self, t: T) -> Point
    where
        f64: From<T>,
    {
        self.origin + self.direction * t
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// // Translating a ray
    /// let ray = Ray {
    ///     origin: point![1, 2, 3],
    ///     direction: vector![0, 1, 0],
    /// };
    /// let transform = Mat4::identity().translate(3, 4, 5);
    /// let ray_transformed = ray.transform(transform);
    /// assert_eq!(ray_transformed.origin, point![4, 6, 8]);
    /// assert_eq!(ray_transformed.direction, vector![0, 1, 0]);
    ///
    /// // Scaling a ray
    /// let ray = Ray {
    ///     origin: point![1, 2, 3],
    ///     direction: vector![0, 1, 0],
    /// };
    /// let transform = Mat4::identity().scale(2, 3, 4);
    /// let ray_transformed = ray.transform(transform);
    /// assert_eq!(ray_transformed.origin, point![2, 6, 12]);
    /// assert_eq!(ray_transformed.direction, vector![0, 3, 0]);
    /// ```
    pub fn transform(&self, transform: Mat4) -> Self {
        Ray {
            origin: transform.clone() * self.origin,
            direction: transform * self.direction,
        }
    }
}
