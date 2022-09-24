use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Mat4;
use crate::point::Point;
use crate::prelude::EPSILON;
use crate::prelude::OBJECT_COUNTER;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vector::Vector;
use std::rc::Rc;
use std::sync::atomic::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    pub id: usize,
    pub transform: Mat4,
    pub material: Material,
}

impl Shape for Plane {
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// // The normal of a plane is constant everywhere
    /// let plane = Plane::default();
    /// assert_eq!(plane.local_normal_at(point![0, 0, 0]), vector![0, 1, 0]);
    /// assert_eq!(plane.local_normal_at(point![10, 0, -10]), vector![0, 1, 0]);
    /// assert_eq!(plane.local_normal_at(point![-5, 0, 150]), vector![0, 1, 0]);
    /// ```
    fn local_normal_at(&self, _point: Point) -> Vector {
        Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// # use std::rc::Rc;
    /// // Intersect with a ray parallel to the plane
    /// let plane = Rc::new(Plane::default());
    /// let ray = Ray {
    ///     origin: point![0, 10, 0],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let intersections = plane.local_intersect(ray);
    /// assert!(intersections.is_empty());
    ///
    /// // Intersect with a coplanar ray
    /// let plane = Plane::default();
    /// let ray = Ray {
    ///     origin: point![0, 0, 0],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let intersections = plane.local_intersect(ray);
    /// assert!(intersections.is_empty());
    ///
    /// // A ray intersecting a plane from above
    /// let plane = Rc::new(Plane::default());
    /// let ray = Ray {
    ///     origin: point![0, 1, 0],
    ///     direction: vector![0, -1, 0],
    /// };
    /// let intersections = plane.local_intersect(ray);
    /// assert_eq!(intersections.len(), 1);
    /// assert!(is_equal(intersections[0].t, 1.0));
    /// // assert_eq!(*intersections[0].object, *plane);
    ///
    /// // A ray intersecting a plane from below
    /// let plane = Rc::new(Plane::default());
    /// let ray = Ray {
    ///     origin: point![0, -1, 0],
    ///     direction: vector![0, 1, 0],
    /// };
    /// assert_eq!(intersections.len(), 1);
    /// assert!(is_equal(intersections[0].t, 1.0));
    /// // assert_eq!(*intersections[0].object, *plane);
    /// ```
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        if ray.direction.y.abs() < EPSILON {
            return Vec::new();
        } else {
            vec![Intersection {
                t: -ray.origin.y / ray.direction.y,
                object: Rc::new(self.clone()),
            }]
        }
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

impl Default for Plane {
    fn default() -> Self {
        Self {
            id: OBJECT_COUNTER.fetch_add(1, Ordering::Relaxed),
            transform: Mat4::identity(),
            material: Material::new(),
        }
    }
}
