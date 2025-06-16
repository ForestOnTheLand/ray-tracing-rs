//! This project aims to implement a basic rendering algorithm in pure rust.
//! Reference: <https://raytracing.github.io/books/RayTracingInOneWeekend.html>

/// Defines the configuration of camera.
pub mod camera;
/// Defines entities in the world.
pub mod entity;
/// Defines the ray.
pub mod ray;
/// Some useful tools.
pub mod utils;

use crate::entity::{Entity, Lambertian, Metal, Sphere};
use nalgebra as na;

fn main() {
    // Set Camera.
    let cam = camera::Camera::from_width_height(400, 225);

    // Set World.
    let world = vec![
        Entity::new(
            Box::new(Sphere::new(100., na::point![0., -100.5, -1.])),
            Box::new(Lambertian::new(na::vector![0.8, 0.8, 0.0])),
        ),
        Entity::new(
            Box::new(Sphere::new(0.5, na::point![0., 0., -1.2])),
            Box::new(Metal::new(na::vector![0.1, 0.2, 0.5])),
        ),
        Entity::new(
            Box::new(Sphere::new(0.5, na::point![-1., 0., -1.])),
            Box::new(Metal::new(na::vector![0.8, 0.8, 0.8])),
        ),
        Entity::new(
            Box::new(Sphere::new(0.5, na::point![1., 0., -1.])),
            Box::new(Metal::new(na::vector![0.8, 0.6, 0.2])),
        ),
    ];

    // Render and Output.
    let image_buf = cam.render_world(&world);
    image_buf.save("image/image.png").unwrap();
}
