use crate::{Point3, Vector3};
use bvh::ray::Ray;

pub struct Camera {
    origin: Point3<f32>,
    lower_left_corner: Point3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
}

impl Default for Camera {
    fn default() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, -viewport_height, 0.0);
        return Camera {
            origin: origin.clone(),
            horizontal,
            vertical,
            lower_left_corner: origin.clone()
                - &horizontal / 2.0
                - &vertical / 2.0
                - Vector3::new(0.0, 0.0, focal_length),
        };
    }
}

impl Camera {
    pub fn new() -> Camera {
        Default::default()
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin.clone(),
            self.lower_left_corner.clone()
                + u * self.horizontal.clone()
                + v * self.vertical.clone()
                - self.origin.clone(),
        )
    }
}

// class camera {
//     public:
//         camera() {
//             auto aspect_ratio = 16.0 / 9.0;
//             auto viewport_height = 2.0;
//             auto viewport_width = aspect_ratio * viewport_height;
//             auto focal_length = 1.0;
//
//             origin = point3(0, 0, 0);
//             horizontal = vec3(viewport_width, 0.0, 0.0);
//             vertical = vec3(0.0, viewport_height, 0.0);
//             lower_left_corner = origin - horizontal / 2 - vertical / 2 - vec3(0, 0, focal_length);
//         }
//
//         ray get_ray(double u, double v) const {
//             return ray(origin, lower_left_corner + u * horizontal + v * vertical - origin);
//         }
//
//     private:
//         point3 origin;
//         point3 lower_left_corner;
//         vec3 horizontal;
//         vec3 vertical;
// };
