use crate::prelude::is_equal;
use crate::sphere::Sphere;
use std::cmp::Ordering;

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// let sphere = Sphere::new();
/// let intersection = Intersection {
///     t: 3.5,
///     object: &sphere,
/// };
/// assert!(is_equal(intersection.t, 3.5));
/// assert_eq!(*intersection.object, sphere);
/// ```
#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        is_equal(self.t, other.t) && self.object == other.object
    }
}

impl PartialOrd for Intersection<'_> {
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

impl Intersection<'_> {
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// // The hit, when all intersections have positive `t`
    /// let sphere = Sphere::new();
    /// let mut intersections = vec![
    ///     Intersection {
    ///         t: 1.0,
    ///         object: &sphere,
    ///     },
    ///     Intersection {
    ///         t: 2.0,
    ///         object: &sphere,
    ///     },
    /// ];
    /// intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
    /// assert_eq!(
    ///     Intersection::hit(&intersections),
    ///     Some(Intersection {
    ///         t: 1.0,
    ///         object: &sphere
    ///     })
    /// );
    ///
    /// // The hit, when some intersections have negative `t`
    /// let sphere = Sphere::new();
    /// let mut intersections = vec![
    ///     Intersection {
    ///         t: -1.0,
    ///         object: &sphere,
    ///     },
    ///     Intersection {
    ///         t: 1.0,
    ///         object: &sphere,
    ///     },
    /// ];
    /// intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
    /// assert_eq!(
    ///     Intersection::hit(&intersections),
    ///     Some(Intersection {
    ///         t: 1.0,
    ///         object: &sphere,
    ///     })
    /// );
    ///
    /// // The hit, when all intersections have negative `t`
    /// let sphere = Sphere::new();
    /// let mut intersections = vec![
    ///     Intersection {
    ///         t: -2.0,
    ///         object: &sphere,
    ///     },
    ///     Intersection {
    ///         t: -1.0,
    ///         object: &sphere,
    ///     },
    /// ];
    /// intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
    /// assert_eq!(Intersection::hit(&intersections), None);
    ///
    /// // The hit is always the lowest nonnegative intersection
    /// let sphere = Sphere::new();
    /// let mut intersections = vec![
    ///     Intersection {
    ///         t: 5.0,
    ///         object: &sphere,
    ///     },
    ///     Intersection {
    ///         t: 7.0,
    ///         object: &sphere,
    ///     },
    ///     Intersection {
    ///         t: -3.0,
    ///         object: &sphere,
    ///     },
    ///     Intersection {
    ///         t: -2.0,
    ///         object: &sphere,
    ///     },
    /// ];
    /// intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
    /// assert_eq!(
    ///     Intersection::hit(&intersections),
    ///     Some(Intersection {
    ///         t: 5.0,
    ///         object: &sphere,
    ///     })
    /// );
    /// ```
    pub fn hit<'a>(intersections: &'a [Intersection]) -> Option<Intersection<'a>> {
        match intersections
            .iter()
            .find(|&intersection| intersection.t > 0.0 || is_equal(intersection.t, 0.0))
        {
            Some(intersection) => Some(intersection.clone()),
            None => None,
        }
    }
}
