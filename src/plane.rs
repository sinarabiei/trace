use crate::intersection::Intersection;
use crate::mat4::Mat4;
use crate::material::Material;
use crate::pattern::Pattern;
use crate::point::Point;
use crate::prelude::EPSILON;
use crate::prelude::OBJECT_COUNTER;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vector::Vector;
use std::sync::atomic::Ordering;

#[derive(Debug, PartialEq)]
pub struct Plane {
    pub id: usize,
    pub transform: Mat4,
    pub material: Material,
}

impl Plane {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;

        self
    }

    pub fn set_pattern(mut self, pattern: Box<dyn Pattern>) -> Self {
        self.material.pattern = Some(pattern);

        self
    }
}

impl Shape for Plane {
    /// The normal of a plane is constant everywhere
    fn local_normal_at(&self, _point: Point) -> Vector {
        Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        if ray.direction.y.abs() < EPSILON {
            return Vec::new();
        } else {
            vec![Intersection {
                t: -ray.origin.y / ray.direction.y,
                object: self,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::is_equal;
    use crate::{point, vector};

    #[test]
    fn test_local_normal_at() {
        // The normal of a plane is constant everywhere
        let plane = Plane::default();
        assert_eq!(plane.local_normal_at(point![0, 0, 0]), vector![0, 1, 0]);
        assert_eq!(plane.local_normal_at(point![10, 0, -10]), vector![0, 1, 0]);
        assert_eq!(plane.local_normal_at(point![-5, 0, 150]), vector![0, 1, 0]);
    }

    #[test]
    fn test_local_intersect() {
        // Intersect with a ray parallel to the plane
        let plane = Plane::default();
        let ray = Ray {
            origin: point![0, 10, 0],
            direction: vector![0, 0, 1],
        };
        let intersections = plane.local_intersect(ray);
        assert!(intersections.is_empty());

        // Intersect with a coplanar ray
        let plane = Plane::default();
        let ray = Ray {
            origin: point![0, 0, 0],
            direction: vector![0, 0, 1],
        };
        let intersections = plane.local_intersect(ray);
        assert!(intersections.is_empty());

        // A ray intersecting a plane from above
        let plane = Plane::default();
        let ray = Ray {
            origin: point![0, 1, 0],
            direction: vector![0, -1, 0],
        };
        let intersections = plane.local_intersect(ray);
        assert_eq!(intersections.len(), 1);
        assert!(is_equal(intersections[0].t, 1.0));
        assert_eq!(intersections[0].object.id(), plane.id);

        // A ray intersecting a plane from below
        let plane = Plane::default();
        let ray = Ray {
            origin: point![0, -1, 0],
            direction: vector![0, 1, 0],
        };
        let intersections = plane.local_intersect(ray);
        assert_eq!(intersections.len(), 1);
        assert!(is_equal(intersections[0].t, 1.0));
        assert_eq!(intersections[0].object.id(), plane.id);
    }
}
