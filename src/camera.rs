use crate::canvas::Canvas;
use crate::mat4::Mat4;
use crate::point::Point;
use crate::ray::Ray;
use crate::world::World;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Mat4,
    pub half_height: f64,
    pub half_width: f64,
    pub pixel_size: f64,
}

impl Camera {
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// # use std::f64::consts::PI;
    /// let camera = Camera::new(160, 120, PI / 2.0);
    /// assert_eq!(camera.hsize, 160);
    /// assert_eq!(camera.vsize, 120);
    /// assert_eq!(camera.field_of_view, PI / 2.0);
    /// assert_eq!(camera.transform, Mat4::identity());
    ///
    /// // The pixel size for a horizontal canvas
    /// let camera = Camera::new(200, 125, PI / 2.0);
    /// assert!(is_equal(camera.pixel_size, 0.01));
    ///
    /// // The pixel size for a vertical canvas
    /// let camera = Camera::new(125, 200, PI / 2.0);
    /// assert!(is_equal(camera.pixel_size, 0.01));
    /// ```
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let transform = Mat4::identity();
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let half_width;
        let half_height;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.0) / hsize as f64;
        Self {
            hsize,
            vsize,
            field_of_view,
            transform,
            half_height,
            half_width,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let pixel = self.transform.inverse()
            * Point {
                x: world_x,
                y: world_y,
                z: -1.0,
            };
        let origin = self.transform.inverse()
            * Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        let direction = (pixel - origin).normalize();
        Ray { origin, direction }
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// # use std::f64::consts::PI;
    /// // Rendering a world with a camera
    /// let world = World::default();
    /// let mut camera = Camera::new(11, 11, PI / 2.0);
    /// let from = point![0, 0, -5];
    /// let to = point![0, 0, 0];
    /// let up = vector![0, 1, 0];
    /// camera.transform = Mat4::identity().view_transform(from, to, up);
    /// let image = camera.render(&world);
    /// assert_eq!(image[(5, 5)], color![0.38066, 0.47583, 0.2855]);
    /// ```
    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..(self.vsize) {
            for x in 0..(self.hsize) {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                image[(x, y)] = color;
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point;
    use crate::{vector, vector::Vector};
    use std::f64::consts::PI;
    use std::f64::consts::SQRT_2;

    #[test]
    fn test_ray_for_pixel() {
        // Constructing a ray through the center of the canvas
        let camera = Camera::new(201, 101, PI / 2.0);
        let ray = camera.ray_for_pixel(100, 50);
        assert_eq!(ray.origin, point![0, 0, 0]);
        assert_eq!(ray.direction, vector![0, 0, -1]);

        // Constructing a ray through a corner of the canvas
        let camera = Camera::new(201, 101, PI / 2.0);
        let ray = camera.ray_for_pixel(0, 0);
        assert_eq!(ray.origin, point![0, 0, 0]);
        assert_eq!(ray.direction, vector![0.66519, 0.33259, -0.66851]);

        // Constructing a ray when the camera is transformed
        let mut camera = Camera::new(201, 101, PI / 2.0);
        camera.transform = Mat4::identity().translate(0, -2, 5).rotate_y(PI / 4.0);
        let ray = camera.ray_for_pixel(100, 50);
        assert_eq!(ray.origin, point![0, 2, -5]);
        assert_eq!(ray.direction, vector![SQRT_2 / 2.0, 0, -SQRT_2 / 2.0]);
    }
}
