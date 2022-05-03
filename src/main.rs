#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod camera;
mod material;
mod ray_ext;
mod sphere;
mod surface;
mod utilities;
mod world;

use crate::camera::Camera;
use crate::material::{LambertianMaterial, MetalMaterial};
use crate::surface::{HittableList, Sphere};
use crate::world::World;
use bvh::nalgebra::Point3;
use bvh::nalgebra::Vector3;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

// Image
static ASPECT_RATIO: f32 = 16.0 / 9.0;
static WIDTH: u32 = 800;
static HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
static MAX_DEPTH: i32 = 50;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("AT2")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let samples_per_pixel = 100;
    let camera = Camera::new();
    let world = World { samples_per_pixel };

    let material_ground = Box::new(LambertianMaterial {
        albedo: Vector3::new(0.8, 0.8, 0.0),
    });
    let material_center = Box::new(LambertianMaterial {
        albedo: Vector3::new(0.7, 0.3, 0.3),
    });
    let material_left = Box::new(MetalMaterial::new(Vector3::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Box::new(MetalMaterial::new(Vector3::new(0.8, 0.6, 0.2), 1.0));

    let objects = HittableList {
        objects: vec![
            Box::new(Sphere {
                center: Point3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: material_center,
            }),
            Box::new(Sphere {
                center: Point3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: material_ground,
            }),
            Box::new(Sphere {
                center: Point3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: material_left,
            }),
            Box::new(Sphere {
                center: Point3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: material_right,
            }),
        ],
    };

    let mut render_done = false;
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }

        if !render_done {
            world.draw(pixels.get_frame(), &camera, &objects);
            render_done = true;
        }
    });
}
