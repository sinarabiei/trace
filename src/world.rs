use crate::color::Color;
use crate::intersection::Computation;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::matrix::Mat4;
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct World {
    pub light: Light,
    pub objects: Vec<Sphere>,
}

impl World {
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// let light = Light {
    ///     position: Point {
    ///         x: -10.0,
    ///         y: 10.0,
    ///         z: -10.0,
    ///     },
    ///     intensity: Color {
    ///         red: 1.0,
    ///         green: 1.0,
    ///         blue: 1.0,
    ///     },
    /// };
    /// let mut sphere_outer = Sphere::new();
    /// let mut sphere_inner = Sphere::new();
    /// sphere_outer.material.color = Color {
    ///     red: 0.8,
    ///     green: 1.0,
    ///     blue: 0.6,
    /// };
    /// sphere_outer.material.diffuse = 0.7;
    /// sphere_outer.material.specular = 0.2;
    /// sphere_inner.transform = Mat4::identity().scale(0.5, 0.5, 0.5);
    /// let world = World::new();
    /// assert_eq!(world.light, light);
    /// // This two do not pass, because each sphere has a unique id.
    /// // assert!(world.objects.contains(&sphere_outer));
    /// // assert!(world.objects.contains(&sphere_inner));
    /// ```
    pub fn new() -> Self {
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
        let mut sphere_outer = Sphere::new();
        let mut sphere_inner = Sphere::new();
        sphere_outer.material.color = Color {
            red: 0.8,
            green: 1.0,
            blue: 0.6,
        };
        sphere_outer.material.diffuse = 0.7;
        sphere_outer.material.specular = 0.2;
        sphere_inner.transform = Mat4::identity().scale(0.5, 0.5, 0.5);

        Self {
            light,
            objects: vec![sphere_outer, sphere_inner],
        }
    }

    /// Intersects a world with a ray.
    /// Returned vector of intersections is sorted.
    ///
    /// # Examples
    /// ```
    /// # use trace::prelude::*;
    /// let world = World::new();
    /// let ray = Ray {
    ///     origin: Point {x: 0.0, y: 0.0, z: -5.0},
    ///     direction: Vector {x: 0.0, y: 0.0, z: 1.0},
    /// };
    /// let intersections = world.intersect(ray);
    /// assert_eq!(intersections.len(), 4);
    /// assert!(is_equal(intersections[0].t, 4.0));
    /// assert!(is_equal(intersections[1].t, 4.5));
    /// assert!(is_equal(intersections[2].t, 5.5));
    /// assert!(is_equal(intersections[3].t, 6.0));
    /// ```
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for object in &self.objects {
            intersections.extend_from_slice(object.intersect(ray).as_slice());
        }
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
        intersections
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// // Shading an intersection
    /// let world = World::new();
    /// let ray = Ray {
    ///     origin: point![0, 0, -5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let shape = &world.objects[0];
    /// let intersection = Intersection {
    ///     t: 4.0,
    ///     object: shape,
    /// };
    /// let comps = intersection.prepare(ray);
    /// assert_eq!(world.shade_hit(comps), color![0.38066, 0.47583, 0.2855]);
    ///
    /// // Shading an intersection from the inside
    /// let mut world = World::new();
    /// world.light = Light {
    ///     position: point![0, 0.25, 0],
    ///     intensity: color![1, 1, 1],
    /// };
    /// let ray = Ray {
    ///     origin: point![0, 0, 0],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let shape = &world.objects[1];
    /// let intersection = Intersection {
    ///     t: 0.5,
    ///     object: shape,
    /// };
    /// let comps = intersection.prepare(ray);
    /// assert_eq!(world.shade_hit(comps), color![0.90498, 0.90498, 0.90498]);
    /// ```
    pub fn shade_hit(&self, comps: Computation) -> Color {
        comps
            .object
            .material
            .lighting(self.light, comps.point, comps.eyev, comps.normal)
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// // The color when a ray misses
    /// let world = World::new();
    /// let ray = Ray {
    ///     origin: point![0, 0, -5],
    ///     direction: vector![0, 1, 0],
    /// };
    /// assert_eq!(world.color_at(ray), color![0, 0, 0]);
    ///
    /// // The color when a ray hits
    /// let world = World::new();
    /// let ray = Ray {
    ///     origin: point![0, 0, -5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// assert_eq!(world.color_at(ray), color![0.38066, 0.47583, 0.2855]);
    ///
    /// // The color with an intersection behind the ray
    /// let mut world = World::new();
    /// world.objects[0].material.ambient = 1.0;
    /// world.objects[1].material.ambient = 1.0;
    /// let outer = &world.objects[0];
    /// let inner = &world.objects[1];
    /// let ray = Ray {
    ///     origin: point![0, 0, 0.75],
    ///     direction: vector![0, 0, -1],
    /// };
    /// assert_eq!(world.color_at(ray), inner.material.color);
    /// ```
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
