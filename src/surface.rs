use crate::ray_ext::RayExt;
use bvh::nalgebra::{Point3, Vector3};
use bvh::ray::Ray;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub point: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
        }
    }
}

impl HitRecord {
    pub fn new() {
        Default::default()
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f32>) {
        self.front_face = r.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            front_face: false,
            t: 0.0,
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = &r.origin - &self.center;
        let a = r.direction.norm_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point = r.at(rec.t);
        rec.normal = (&rec.point - &self.center) / self.radius;

        let outward_normal = (&rec.point - &self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        return true;
    }
}
