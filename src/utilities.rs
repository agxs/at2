use crate::Vector3;
use bvh::nalgebra::{magnitude_squared, norm_squared, Matrix};
use rand::Rng;

static PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    return x;
}

pub fn unit_vector(v: &Vector3<f32>) -> Vector3<f32> {
    return v / Matrix::magnitude(&v);
}

pub fn random_vec3() -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

pub fn random_vec3_range(min: f32, max: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    Vector3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        if Matrix::magnitude_squared(&p) >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vector3<f32> {
    return unit_vector(&random_in_unit_sphere());
}

pub fn random_in_hemisphere(normal: &Vector3<f32>) -> Vector3<f32> {
    let in_unit_sphere = random_in_unit_sphere();
    if Matrix::dot(&in_unit_sphere, normal) > 0.0 {
        // In the same hemisphere as the normal
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}
