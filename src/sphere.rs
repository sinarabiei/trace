use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Mat4;
use crate::point;
use crate::point::Point;
use crate::prelude::is_equal;
use crate::prelude::OBJECT_COUNTER;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vector::Vector;
use std::rc::Rc;
use std::sync::atomic::Ordering;

/// `Sphere` instances are situated at the world's origin (0, 0, 0),
/// and are all unit spheres, with radius of 1.
#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub id: usize,
    pub transform: Mat4,
    pub material: Material,
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
        Self {
            id: OBJECT_COUNTER.fetch_add(1, Ordering::Relaxed),
            transform: Mat4::identity(),
            material: Material::new(),
        }
    }
}

// TODO
// impl Default for Sphere {}

impl Shape for Sphere {
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// # use std::rc::Rc;
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
    /// let sphere = Rc::new(Sphere::new());
    /// let intersections = sphere.intersect(ray);
    /// assert_eq!(intersections.len(), 2);
    /// // assert_eq!(*intersections[0].object, *sphere);
    /// // assert_eq!(*intersections[1].object, *sphere);
    ///
    /// // Intersecting a scaled sphere with a ray
    /// let ray = Ray {
    ///     origin: point![0, 0, -5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let mut sphere = Rc::new(Sphere::new());
    /// Rc::get_mut(&mut sphere).unwrap().transform = Mat4::identity().scale(2, 2, 2);
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
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        // the vector from the sphere's center, to the ray origin
        // remember: the sphere is centered at the world origin
        let sphere_to_ray = local_ray.origin - point![0, 0, 0];
        let a = local_ray.direction.dot(local_ray.direction);
        let b = 2.0 * local_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            Vec::new()
        } else if is_equal(discriminant, 0.0) {
            vec![Intersection {
                t: (-b - discriminant.sqrt()) / (2.0 * a),
                object: Rc::new(self.clone()),
            }]
        } else {
            vec![
                Intersection {
                    t: (-b - discriminant.sqrt()) / (2.0 * a),
                    object: Rc::new(self.clone()),
                },
                Intersection {
                    t: (-b + discriminant.sqrt()) / (2.0 * a),
                    object: Rc::new(self.clone()),
                },
            ]
        }
    }

    /// It assumes that the point will always
    /// be on the surface of the sphere.
    ///
    /// # Example
    ///
    /// ```
    /// # use trace::prelude::*;
    /// # use std::f64::consts::PI;
    /// # use std::f64::consts::SQRT_2;
    /// // The normal on a sphere at a point on the x axis
    /// let sphere = Sphere::new();
    /// assert_eq!(sphere.normal_at(point![1, 0, 0]), vector![1, 0, 0]);
    ///
    /// // The normal on a sphere at a point on the y axis
    /// let sphere = Sphere::new();
    /// assert_eq!(sphere.normal_at(point![0, 1, 0]), vector![0, 1, 0]);
    ///
    /// // The normal on a sphere at a point on the z axis
    /// let sphere = Sphere::new();
    /// assert_eq!(sphere.normal_at(point![0, 0, 1]), vector![0, 0, 1]);
    ///
    /// // The normal on a sphere at a nonaxial point
    /// let sphere = Sphere::new();
    /// assert_eq!(
    ///     sphere.normal_at(point![3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0]),
    ///     vector![3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0]
    /// );
    ///
    /// // The normal is a normalized vector
    /// let sphere = Sphere::new();
    /// let normal = sphere.normal_at(point![3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0]);
    /// assert_eq!(normal.normalize(), normal);
    ///
    /// // Computing the normal on a translated sphere
    /// let mut sphere = Sphere::new();
    /// sphere.transform = Mat4::identity().translate(0, 1, 0);
    /// assert_eq!(
    ///     sphere.normal_at(point![0, 1.70711, -0.70711]),
    ///     vector![0, 0.70711, -0.70711]
    /// );
    ///
    /// // Computing the normal on a transformed sphere
    /// let mut sphere = Sphere::new();
    /// sphere.transform = Mat4::identity().rotate_z(PI / 5.0).scale(1, 0.5, 1);
    /// assert_eq!(
    ///     sphere.normal_at(point![0, SQRT_2 / 2.0, -SQRT_2 / 2.0]),
    ///     vector![0, 0.97014, -0.24254]
    /// );
    /// ```
    fn local_normal_at(&self, local_point: Point) -> Vector {
        local_point - Point::zero()
    }

    fn transform(&self) -> &Mat4 {
        &self.transform
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn debug(&self) -> String {
        format!("{:?}", self)
    }

    fn id(&self) -> usize {
        self.id
    }
}
