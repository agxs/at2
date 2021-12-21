use bvh::nalgebra::Point3;
use bvh::ray::Ray;

pub trait RayExt {
    fn at(&self, t: f32) -> Point3<f32>;
}

impl RayExt for Ray {
    fn at(&self, t: f32) -> Point3<f32> {
        &self.origin + t * &self.direction
    }
}
