use crate::ray_ext::RayExt;
use crate::sphere::hit_sphere;
use crate::surface::{HitRecord, Hittable, HittableList};
use crate::utilities::{clamp, random_in_unit_sphere};
use crate::{Camera, HEIGHT, MAX_DEPTH, WIDTH};
use bvh::nalgebra::Point3;
use bvh::nalgebra::Vector3;
use bvh::ray::Ray;
use rand::Rng;
use std::f32::INFINITY;

pub struct World {
    pub samples_per_pixel: i32,
}

impl World {
    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    pub fn draw(&self, frame: &mut [u8], camera: &Camera, objects: &HittableList) {
        let mut rng = rand::thread_rng();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // convert flat array into 2d pixel coordinates
            let x = (i % WIDTH as usize) as f32;
            let y = (i / WIDTH as usize) as f32;

            let mut color = Vector3::new(0.0, 0.0, 0.0);
            for s in 0..self.samples_per_pixel {
                // u, v are between 0 and 1 for each dimension through x and y
                let u = (x + rng.gen::<f32>()) / (WIDTH as f32 - 1.0);
                let v = (y + rng.gen::<f32>()) / (HEIGHT as f32 - 1.0);
                let r = camera.get_ray(u, v);
                color += self.ray_color(&r, objects, MAX_DEPTH);
            }
            self.write_color(pixel, &color, self.samples_per_pixel);
        }
    }

    fn ray_color(&self, r: &Ray, objects: &dyn Hittable, depth: i32) -> Vector3<f32> {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        let mut rec: HitRecord = HitRecord {
            ..Default::default()
        };
        if objects.hit(r, 0.001, INFINITY, &mut rec) {
            let target = rec.point + rec.normal + random_in_unit_sphere();
            let r = Ray::new(rec.point.clone(), target - rec.point.clone());
            // return 0.5 * ray_color(&r, &objects);
            return 0.5 * self.ray_color(&r, objects, depth - 1);

            // let n = rec.normal + Vector3::new(1.0, 1.0, 1.0); // colour 1, 1, 1
            // return 0.5 * n;
        }

        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return t * Vector3::new(1.0, 1.0, 1.0) + (1.0 - t) * Vector3::new(0.5, 0.7, 1.0);
    }

    fn write_color(&self, pixel: &mut [u8], color: &Vector3<f32>, samples_per_pixel: i32) {
        let mut r = color[0];
        let mut g = color[1];
        let mut b = color[2];

        // Divide the color by the number of samples.
        let scale = 1.0 / (samples_per_pixel as f32);
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        // Write the translated [0,255] value of each color component.
        pixel[0] = (256.0 * clamp(r, 0.0, 0.999)) as u8;
        pixel[1] = (256.0 * clamp(g, 0.0, 0.999)) as u8;
        pixel[2] = (256.0 * clamp(b, 0.0, 0.999)) as u8;
        pixel[3] = 255;
    }
}
