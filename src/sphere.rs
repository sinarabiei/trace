use crate::intersection::Intersection;
use crate::matrix::Mat4;
use crate::point;
use crate::point::Point;
use crate::prelude::is_equal;
use crate::ray::Ray;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: usize,
    pub transform: Mat4,
}

impl Sphere {
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// // A sphere's default transformation
    /// let mut sphere = Sphere::new();
    /// assert_eq!(sphere.transform, Mat4::identity());
    ///
    /// // Changing a sphere's transformation
    /// let mut sphere = Sphere::new();
    /// sphere.transform = Mat4::identity().translate(2, 3, 4);
    /// assert_eq!(sphere.transform, Mat4::identity().translate(2, 3, 4));
    /// ```
    pub fn new() -> Sphere {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            transform: Mat4::identity(),
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// // A ray intersects a sphere at two points
    /// let ray = Ray {
    ///     origin: point![0, 0, -5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(ray);
    /// assert_eq!(intersections.len(), 2);
    /// assert!(is_equal(intersections[0].t, 4.0));
    /// assert!(is_equal(intersections[1].t, 6.0));
    ///
    /// // A ray intersects a sphere at a tangent
    /// let ray = Ray {
    ///     origin: point![0, 1, -5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(ray);
    /// assert_eq!(intersections.len(), 1);
    /// assert!(is_equal(intersections[0].t, 5.0));
    ///
    /// // A ray misses a sphere
    /// let ray = Ray {
    ///     origin: point![0, 2, -5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(ray);
    /// assert_eq!(intersections.len(), 0);
    ///
    /// // A ray originates inside a sphere
    /// let ray = Ray {
    ///     origin: point![0, 0, 0],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(ray);
    /// assert_eq!(intersections.len(), 2);
    /// assert!(is_equal(intersections[0].t, -1.0));
    /// assert!(is_equal(intersections[1].t, 1.0));
    ///
    /// // A sphere is behind a ray
    /// let ray = Ray {
    ///     origin: point![0, 0, 5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(ray);
    /// assert_eq!(intersections.len(), 2);
    /// assert!(is_equal(intersections[0].t, -6.0));
    /// assert!(is_equal(intersections[1].t, -4.0));
    ///
    /// // Intersect sets the object on the intersection
    /// let ray = Ray {
    ///     origin: point![0, 0, -5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let sphere = Sphere::new();
    /// let intersections = sphere.intersect(ray);
    /// assert_eq!(intersections.len(), 2);
    /// assert_eq!(*intersections[0].object, sphere);
    /// assert_eq!(*intersections[1].object, sphere);
    ///
    /// // Intersecting a scaled sphere with a ray
    /// let ray = Ray {
    ///     origin: point![0, 0, -5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let mut sphere = Sphere::new();
    /// sphere.transform = Mat4::identity().scale(2, 2, 2);
    /// let intersections = sphere.intersect(ray);
    /// assert_eq!(intersections.len(), 2);
    /// assert!(is_equal(intersections[0].t, 3.0));
    /// assert!(is_equal(intersections[1].t, 7.0));
    ///
    /// // Intersecting a translated sphere with a ray
    /// let ray = Ray {
    ///     origin: point![0, 0, -5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let mut sphere = Sphere::new();
    /// sphere.transform = Mat4::identity().translate(5, 0, 0);
    /// let intersections = sphere.intersect(ray);
    /// assert_eq!(intersections.len(), 0);
    /// ```
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        // the vector from the sphere's center, to the ray origin
        // remember: the sphere is centered at the world origin
        let ray = ray.transform(self.transform.inverse());
        let sphere_to_ray = ray.origin - point![0, 0, 0];
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            Vec::new()
        } else if is_equal(discriminant, 0.0) {
            vec![Intersection {
                t: (-b - discriminant.sqrt()) / (2.0 * a),
                object: self,
            }]
        } else {
            vec![
                Intersection {
                    t: (-b - discriminant.sqrt()) / (2.0 * a),
                    object: self,
                },
                Intersection {
                    t: (-b + discriminant.sqrt()) / (2.0 * a),
                    object: self,
                },
            ]
        }
    }
}
