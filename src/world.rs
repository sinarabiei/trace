use crate::color::Color;
use crate::intersection::Computation;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::matrix::Mat4;
use crate::point::Point;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;
use std::rc::Rc;

pub struct World {
    pub light: Light,
    pub objects: Vec<Rc<dyn Shape>>,
}

impl World {
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
            objects: vec![Rc::new(sphere_outer), Rc::new(sphere_inner)],
        }
    }

    /// Intersects a world with a ray.
    /// Returned vector of intersections is sorted.
    ///
    /// # Examples
    ///
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
    /// // There is no shadow when nothing is collinear with point and light
    /// let world = World::new();
    /// let point = point![0, 10, 0];
    /// assert_eq!(world.is_shadowed(point), false);
    ///
    /// // The shadow when an object is between the point and the light
    /// let world = World::new();
    /// let point = point![10, -10, 10];
    /// assert_eq!(world.is_shadowed(point), true);
    ///
    /// // There is no shadow when an object is behind the light
    /// let world = World::new();
    /// let point = point![-20, 20, -20];
    /// assert_eq!(world.is_shadowed(point), false);
    ///
    /// // There is no shadow when an object is behind the point
    /// let world = World::new();
    /// let point = point![-2, 2, -2];
    /// assert_eq!(world.is_shadowed(point), false);
    /// ```
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

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// # use std::rc::Rc;
    /// // Shading an intersection
    /// let world = World::new();
    /// let ray = Ray {
    ///     origin: point![0, 0, -5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let shape = world.objects[0].clone();
    /// let intersection = Intersection {
    ///     t: 4.0,
    ///     object: shape,
    /// };
    /// let mut comps = intersection.prepare(ray);
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
    ///     object: shape.clone(),
    /// };
    /// let comps = intersection.prepare(ray);
    /// assert_eq!(world.shade_hit(comps), color![0.90498, 0.90498, 0.90498]);
    ///
    /// // shade_hit() is given an intersection in shadow
    /// let mut world = World::new();
    /// world.light = Light {
    ///     position: point![0, 0, -10],
    ///     intensity: color![1, 1, 1],
    /// };
    /// let sphere_one = Rc::new(Sphere::new());
    /// world.objects.push(sphere_one);
    /// let mut sphere_two = Rc::new(Sphere::new());
    /// Rc::get_mut(&mut sphere_two).unwrap().transform = Mat4::identity().translate(0, 0, 10);
    /// world.objects.push(sphere_two);
    /// let ray = Ray {
    ///     origin: point![0, 0, 5],
    ///     direction: vector![0, 0, 1],
    /// };
    /// let intersection = Intersection {
    ///     t: 4.0,
    ///     object: world.objects[1].clone(),
    /// };
    /// let comps = intersection.prepare(ray);
    /// assert_eq!(world.shade_hit(comps), color![0.1, 0.1, 0.1]);
    /// ```
    pub fn shade_hit(&self, comps: Computation) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);
        comps.object.material().lighting(
            self.light,
            comps.over_point,
            comps.eyev,
            comps.normal,
            shadowed,
        )
    }

    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// # use std::rc::Rc;
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
    /// Rc::get_mut(&mut world.objects[0]).unwrap().material_mut().ambient = 1.0;
    /// Rc::get_mut(&mut world.objects[1]).unwrap().material_mut().ambient = 1.0;
    /// let outer = world.objects[0].clone();
    /// let inner = world.objects[1].clone();
    /// let ray = Ray {
    ///     origin: point![0, 0, 0.75],
    ///     direction: vector![0, 0, -1],
    /// };
    /// assert_eq!(world.color_at(ray), inner.material().color);
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
