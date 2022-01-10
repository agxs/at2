use crate::surface::HitRecord;
use crate::utilities::{
    near_zero, random_in_unit_sphere, random_unit_vector, reflect, unit_vector,
};
use crate::Vector3;
use bvh::ray::Ray;

pub trait Material: CloneMaterial {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vector3<f32>,
        _scattered: &mut Ray,
    ) -> bool;
}

pub trait CloneMaterial {
    fn clone_material<'a>(&self) -> Box<dyn Material>;
}

impl<T> CloneMaterial for T
where
    T: Material + Clone + 'static,
{
    fn clone_material(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Self {
        self.clone_material()
    }
}

#[derive(Clone)]
pub struct LambertianMaterial {
    pub albedo: Vector3<f32>,
}

impl Material for LambertianMaterial {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vector3<f32>,
        _scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = &_rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if near_zero(&scatter_direction) {
            scatter_direction.data = _rec.normal.data.clone();
        }

        _scattered.origin = _rec.point.clone();
        _scattered.direction = scatter_direction;
        _attenuation.data = self.albedo.data.clone();
        return true;
    }
}

#[derive(Clone)]
pub struct MetalMaterial {
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}

impl MetalMaterial {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> MetalMaterial {
        MetalMaterial {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vector3<f32>,
        _scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&unit_vector(&_r_in.direction), &_rec.normal);
        _scattered.origin = _rec.point.clone();
        _scattered.direction = reflected + self.fuzz * random_in_unit_sphere();
        _attenuation.data = self.albedo.data.clone();
        return _scattered.direction.dot(&_rec.normal) > 0.0;
    }
}
