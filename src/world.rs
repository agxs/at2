use crate::ray_ext::RayExt;
use crate::sphere::hit_sphere;
use crate::surface::{HitRecord, Hittable, HittableList};
use crate::{HEIGHT, WIDTH};
use bvh::nalgebra::Point3;
use bvh::nalgebra::Vector3;
use bvh::ray::Ray;
use std::f32::INFINITY;

pub struct World {
    pub origin: Point3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub lower_left_corner: Point3<f32>,
}

impl World {
    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    pub fn draw(&self, frame: &mut [u8], world: &HittableList) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            // pixel[0] = ((x as f32 / WIDTH as f32) * 255.0) as u8;
            // pixel[1] = ((y as f32 / HEIGHT as f32) * 255.0) as u8;
            // pixel[2] = (0.25 * 255.0) as u8;
            // pixel[3] = 255;

            // let rgba = if x == y {
            //     [0xff, 0xff, 0xff, 0xff]
            // } else {
            //     [0x00, 0x00, 0x00, 0xff]
            // };

            // pixel.copy_from_slice(&rgba);

            let u = x as f32 / WIDTH as f32;
            let v = y as f32 / HEIGHT as f32;
            let r = Ray::new(
                self.origin.clone(),
                self.lower_left_corner.clone()
                    + u * self.horizontal.clone()
                    + v * self.vertical.clone()
                    - self.origin.clone(),
            );

            let color = self.ray_color(&r, world);
            self.write_color(pixel, &color);
        }
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Vector3<f32> {
        let mut rec: HitRecord = HitRecord {
            ..Default::default()
        };
        if world.hit(r, 0.0, INFINITY, &mut rec) {
            let n = rec.normal + Vector3::new(1.0, 1.0, 1.0); // colour 1, 1, 1
            return 0.5 * n;
        }

        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return t * Vector3::new(1.0, 1.0, 1.0) + (1.0 - t) * Vector3::new(0.5, 0.7, 1.0);
    }

    fn write_color(&self, pixel: &mut [u8], color: &Vector3<f32>) {
        pixel[0] = (color[0] * 255.0) as u8;
        pixel[1] = (color[1] * 255.0) as u8;
        pixel[2] = (color[2] * 255.0) as u8;
        pixel[3] = 255;
    }
}
