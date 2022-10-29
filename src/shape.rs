use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Mat4;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;
use std::fmt::Debug;

pub trait Shape {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.transform().inverse());
        self.local_intersect(local_ray)
    }

    fn normal_at(&self, point: Point) -> Vector {
        let local_point = self.transform().inverse() * point;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = self.transform().inverse().transpose() * local_normal;
        world_normal.normalize()
    }

    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, local_point: Point) -> Vector;
    fn transform(&self) -> &Mat4;
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn debug(&self) -> String;
    fn id(&self) -> usize;
}

impl Debug for dyn Shape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.debug())
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, rhs: &Self) -> bool {
        self.id() == rhs.id()
    }
}
