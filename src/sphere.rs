use bvh::nalgebra::Point3;
use bvh::nalgebra::Vector3;
use bvh::ray::Ray;

pub fn hit_sphere(center: &Point3<f32>, radius: f32, r: &Ray) -> f32 {
    let oc: Vector3<f32> = &r.origin - center;

    let a = r.direction.norm_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.norm_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    return if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    };
}
