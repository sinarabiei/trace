use crate::color::Color;
use crate::intersection::Computation;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::mat4::Mat4;
use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;

pub struct World {
    pub light: Light,
    pub objects: Vec<Box<dyn Shape>>,
}

impl Default for World {
    fn default() -> Self {
        let light = Light {
            position: Point {
                x: -10.0,
                y: 10.0,
                z: -10.0,
            },
            intensity: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
        };

        let sphere_outer = Sphere {
            material: Material {
                color: Color {
                    red: 0.8,
                    green: 1.0,
                    blue: 0.6,
                },
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        };
        let sphere_inner = Sphere {
            transform: Mat4::identity().scale(0.5, 0.5, 0.5),
            ..Default::default()
        };

        Self {
            light,
            objects: vec![Box::new(sphere_outer), Box::new(sphere_inner)],
        }
    }
}

impl World {
    pub fn new(light: Light) -> Self {
        Self {
            light,
            objects: Vec::new(),
        }
    }

    pub fn push<T>(&mut self, object: T)
    where
        T: Shape + 'static,
    {
        self.objects.push(Box::new(object));
    }

    /// Intersects a world with a ray.
    /// Returned vector of intersections is sorted.
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for object in &self.objects {
            for intersection in object.intersect(ray) {
                intersections.push(intersection);
            }
        }
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
        intersections
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        let point_to_light = self.light.position - point;
        let distance = point_to_light.magnitude();
        let direction = point_to_light.normalize();
        let ray = Ray {
            origin: point,
            direction,
        };
        let intersections = self.intersect(ray);
        let hit = Intersection::hit(&intersections);
        if let Some(hit) = hit {
            if hit.t < distance {
                return true;
            }
        }
        false
    }

    pub fn shade_hit(&self, comps: Computation) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);
        comps.object.material().lighting(
            &*comps.object,
            self.light,
            comps.over_point,
            comps.eyev,
            comps.normal,
            shadowed,
        )
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.intersect(ray);
        let hit = Intersection::hit(&intersections);
        match hit {
            Some(hit) => self.shade_hit(hit.prepare(ray)),
            None => Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;
    use crate::intersection::Intersection;
    use crate::point;
    use crate::prelude::is_equal;
    use crate::ray::Ray;
    use crate::{vector, vector::Vector};

    #[test]
    fn test_intersect() {
        let world = World::default();
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let intersections = world.intersect(ray);
        assert_eq!(intersections.len(), 4);
        assert!(is_equal(intersections[0].t, 4.0));
        assert!(is_equal(intersections[1].t, 4.5));
        assert!(is_equal(intersections[2].t, 5.5));
        assert!(is_equal(intersections[3].t, 6.0));
    }

    #[test]
    fn test_is_shadowed() {
        // There is no shadow when nothing is collinear with point and light
        let world = World::default();
        let point = point![0, 10, 0];
        assert_eq!(world.is_shadowed(point), false);

        // The shadow when an object is between the point and the light
        let world = World::default();
        let point = point![10, -10, 10];
        assert_eq!(world.is_shadowed(point), true);

        // There is no shadow when an object is behind the light
        let world = World::default();
        let point = point![-20, 20, -20];
        assert_eq!(world.is_shadowed(point), false);

        // There is no shadow when an object is behind the point
        let world = World::default();
        let point = point![-2, 2, -2];
        assert_eq!(world.is_shadowed(point), false);
    }

    #[test]
    fn test_color_at() {
        // The color when a ray misses
        let world = World::default();
        let ray = Ray {
            origin: point![0, 0, -5],
            direction: vector![0, 1, 0],
        };
        assert_eq!(world.color_at(ray), color![0, 0, 0]);

        // The color when a ray hits
        let world = World::default();
        let ray = Ray {
            origin: point![0, 0, -5],
            direction: vector![0, 0, 1],
        };
        assert_eq!(world.color_at(ray), color![0.38066, 0.47583, 0.2855]);

        // The color with an intersection behind the ray
        let mut world = World::default();
        world.objects[0].material_mut().ambient = 1.0;
        world.objects[1].material_mut().ambient = 1.0;
        let inner = &world.objects[1];
        let ray = Ray {
            origin: point![0, 0, 0.75],
            direction: vector![0, 0, -1],
        };
        assert_eq!(world.color_at(ray), inner.material().color);
    }

    #[test]
    fn test_shade_hit() {
        // Shading an intersection
        let world = World::default();
        let ray = Ray {
            origin: point![0, 0, -5],
            direction: vector![0, 0, 1],
        };
        let shape = &(*world.objects[0]);
        let intersection = Intersection {
            t: 4.0,
            object: shape,
        };
        let comps = intersection.prepare(ray);
        assert_eq!(world.shade_hit(comps), color![0.38066, 0.47583, 0.2855]);

        // Shading an intersection from the inside
        let mut world = World::default();
        world.light = Light {
            position: point![0, 0.25, 0],
            intensity: color![1, 1, 1],
        };
        let ray = Ray {
            origin: point![0, 0, 0],
            direction: vector![0, 0, 1],
        };
        let intersection = Intersection {
            t: 0.5,
            object: &(*world.objects[1]),
        };
        let comps = intersection.prepare(ray);
        assert_eq!(world.shade_hit(comps), color![0.90498, 0.90498, 0.90498]);

        // shade_hit() is given an intersection in shadow
        let mut world = World::default();
        world.light = Light {
            position: point![0, 0, -10],
            intensity: color![1, 1, 1],
        };
        let sphere_one = Sphere::new();
        world.objects.push(Box::new(sphere_one));
        let mut sphere_two = Sphere::new();
        sphere_two.transform = Mat4::identity().translate(0, 0, 10);
        world.objects.push(Box::new(sphere_two));
        let ray = Ray {
            origin: point![0, 0, 5],
            direction: vector![0, 0, 1],
        };
        let intersection = Intersection {
            t: 4.0,
            object: &(*world.objects[1]),
        };
        let comps = intersection.prepare(ray);
        assert_eq!(world.shade_hit(comps), color![0.1, 0.1, 0.1]);
    }
}
