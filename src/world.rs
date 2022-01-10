use crate::surface::{HitRecord, Hittable, HittableList};
use crate::utilities::{clamp, random_in_hemisphere};
use crate::{Camera, Point3, HEIGHT, MAX_DEPTH, WIDTH};
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
            for _s in 0..self.samples_per_pixel {
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

        return match objects.hit(r, 0.001, INFINITY) {
            Some(rec) => {
                let mut scattered =
                    Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
                let mut attenuation = Vector3::new(0.0, 0.0, 0.0);
                if rec
                    .material
                    .scatter(r, &rec, &mut attenuation, &mut scattered)
                {
                    let ray_color = self.ray_color(&scattered, objects, depth - 1);
                    return Vector3::new(
                        attenuation[0] * ray_color[0],
                        attenuation[1] * ray_color[1],
                        attenuation[2] * ray_color[2],
                    );
                }
                return Vector3::new(0.0, 0.0, 0.0);
                // let target = rec.point + random_in_hemisphere(&rec.normal);
                // let r = Ray::new(rec.point.clone(), target - rec.point.clone());
                //
                // 0.5 * self.ray_color(&r, objects, depth - 1)
            }
            None => {
                let unit_direction = r.direction.normalize();
                let t: f32 = 0.5 * (unit_direction.y + 1.0);

                (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
            }
        };
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
        pixel[0] = (255.0 * clamp(r, 0.0, 1.0)) as u8;
        pixel[1] = (255.0 * clamp(g, 0.0, 1.0)) as u8;
        pixel[2] = (255.0 * clamp(b, 0.0, 1.0)) as u8;
        pixel[3] = 255;
    }
}
