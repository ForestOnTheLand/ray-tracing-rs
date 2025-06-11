//! This project aims to implement a basic rendering algorithm in pure rust.

/// Defines the configuration of camera.
mod camera;
/// Defines entities in the world.
mod entity;
/// Defines the ray.
mod ray;
/// Some useful tools.
mod utils;

use crate::entity::{Entities, Entity, Lambertian, Metal, Sphere};
use nalgebra as na;

fn main() {
    // Set Camera.
    let cam = camera::Camera::from_width_height(400, 225);

    // Set World.
    let world = Entities::new(vec![
        Entity::new(
            Box::new(Lambertian::new(na::vector![0.8, 0.8, 0.0])),
            Box::new(Sphere::new(100., na::point![0., -100.5, -1.])),
        ),
        Entity::new(
            Box::new(Metal::new(na::vector![0.1, 0.2, 0.5])),
            Box::new(Sphere::new(0.5, na::point![0., 0., -1.2])),
        ),
        Entity::new(
            Box::new(Metal::new(na::vector![0.8, 0.8, 0.8])),
            Box::new(Sphere::new(0.5, na::point![-1., 0., -1.])),
        ),
        Entity::new(
            Box::new(Metal::new(na::vector![0.8, 0.6, 0.2])),
            Box::new(Sphere::new(0.5, na::point![1., 0., -1.])),
        ),
    ]);

    // Render and Output.
    let image_buf = cam.render_world(&world);
    image_buf.save("image/image.png").unwrap();
}
