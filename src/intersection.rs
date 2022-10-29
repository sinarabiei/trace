use crate::point::Point;
use crate::prelude::is_equal;
use crate::prelude::EPSILON;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vector::Vector;
use std::cmp::Ordering;

pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
}

use std::fmt;

impl<'a> fmt::Debug for Intersection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Intersection {{t: {}}}", self.t)
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        is_equal(self.t, other.t) // && self.object == other.object
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.t < other.t {
            Some(Ordering::Less)
        } else if is_equal(self.t, other.t) {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl<'a> Intersection<'a> {
    pub fn hit(intersections: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
        match intersections
            .iter()
            .find(|&intersection| intersection.t > 0.0 || is_equal(intersection.t, 0.0))
        {
            Some(intersection) => Some(intersection),
            None => None,
        }
    }

    /// Prepares the state of an intersection
    /// to reuse in different calculations.
    pub fn prepare(&self, ray: Ray) -> Computation<'a> {
        let t = self.t;
        let object = self.object;
        let point = ray.position(t);
        let eyev = -ray.direction;
        let mut normal = self.object.normal_at(point);
        let mut inside = false;
        if normal.dot(eyev) < 0.0 {
            inside = true;
            normal = -normal;
        }
        let over_point = point + normal * EPSILON;
        Computation {
            t,
            object,
            point,
            over_point,
            eyev,
            normal,
            inside,
        }
    }
}

pub struct Computation<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
    pub point: Point,
    pub over_point: Point,
    pub eyev: Vector,
    pub normal: Vector,
    pub inside: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mat4::Mat4;
    use crate::sphere::Sphere;
    use crate::{point, point::Point};
    use crate::{vector, vector::Vector};

    #[test]
    fn test_hit() {
        // The hit, when all intersections have positive `t`
        let sphere = Sphere::new();
        let mut intersections = vec![
            Intersection {
                t: 1.0,
                object: &sphere,
            },
            Intersection {
                t: 2.0,
                object: &sphere,
            },
        ];
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            Intersection::hit(&intersections),
            Some(&Intersection {
                t: 1.0,
                object: &sphere,
            })
        );

        // The hit, when some intersections have negative `t`
        let sphere = Sphere::new();
        let mut intersections = vec![
            Intersection {
                t: -1.0,
                object: &sphere,
            },
            Intersection {
                t: 1.0,
                object: &sphere,
            },
        ];
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            Intersection::hit(&intersections),
            Some(&Intersection {
                t: 1.0,
                object: &sphere,
            })
        );

        // The hit, when all intersections have negative `t`
        let sphere = Sphere::new();
        let mut intersections = vec![
            Intersection {
                t: -2.0,
                object: &sphere,
            },
            Intersection {
                t: -1.0,
                object: &sphere,
            },
        ];
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(Intersection::hit(&intersections), None);

        // The hit is always the lowest nonnegative intersection
        let sphere = Sphere::new();
        let mut intersections = vec![
            Intersection {
                t: 5.0,
                object: &sphere,
            },
            Intersection {
                t: 7.0,
                object: &sphere,
            },
            Intersection {
                t: -3.0,
                object: &sphere,
            },
            Intersection {
                t: -2.0,
                object: &sphere,
            },
        ];
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            Intersection::hit(&intersections),
            Some(&Intersection {
                t: 5.0,
                object: &sphere,
            })
        );
    }

    #[test]
    fn test_prepare() {
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
        let shape = Sphere::new();
        let intersection = Intersection {
            t: 4.0,
            object: &shape,
        };
        let comps = intersection.prepare(ray);
        assert!(is_equal(comps.t, intersection.t));
        assert_eq!(comps.point, point![0, 0, -1]);
        assert_eq!(comps.eyev, vector![0, 0, -1]);
        assert_eq!(comps.normal, vector![0, 0, -1]);

        // The hit, when an intersection occurs on the outside
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

        let shape = Sphere::new();
        let intersection = Intersection {
            t: 4.0,
            object: &shape,
        };
        let comps = intersection.prepare(ray);
        assert_eq!(comps.inside, false);

        // The hit, when an intersection occurs on the inside
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let shape = Sphere::new();
        let intersection = Intersection {
            t: 1.0,
            object: &shape,
        };
        let comps = intersection.prepare(ray);
        assert_eq!(comps.point, point![0, 0, 1]);
        assert_eq!(comps.eyev, vector![0, 0, -1]);
        assert_eq!(comps.inside, true);
        // normal would have been (0, 0, 1), but is inverted!
        assert_eq!(comps.normal, vector![0, 0, -1]);

        // The hit should offset the point
        let ray = Ray {
            origin: point![0, 0, -5],
            direction: vector![0, 0, 1],
        };
        let mut shape = Sphere::new();
        shape.transform = Mat4::identity().translate(0, 0, 1);
        let intersection = Intersection {
            t: 5.0,
            object: &shape,
        };
        let comps = intersection.prepare(ray);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
