use crate::material::Material;
use crate::ray_ext::RayExt;
use bvh::nalgebra::{Point3, Vector3};
use bvh::ray::Ray;

#[derive(Clone)]
pub struct HitRecord<'m> {
    pub point: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
    pub material: &'m Box<dyn Material + Sync + Send>,
}

impl<'m> HitRecord<'m> {
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
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Hittable + Sync + Send>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_rec = None;
        let mut closest_so_far = t_max;
        let mut hit_anything = false;

        for object in self.objects.iter() {
            match object.hit(r, t_min, closest_so_far) {
                Some(temp_rec) => {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    closest_rec = Some(temp_rec);
                }
                None => {}
            };
        }

        return closest_rec;
    }
}

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub material: Box<dyn Material + Sync + Send>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.norm_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (&point - &self.center) / self.radius;

        let outward_normal = (&point - &self.center) / self.radius;

        let mut hit = HitRecord {
            point,
            normal,
            t: root,
            front_face: false,
            material: &self.material,
        };
        hit.set_face_normal(ray, outward_normal);

        return Option::Some(hit);
    }
}
