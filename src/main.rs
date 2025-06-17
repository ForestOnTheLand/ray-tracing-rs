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

use crate::entity::{Dielectric, Entity, Lambertian, Material, Metal, Sphere};
use nalgebra as na;
use rand::Rng;

fn main() {
    // Set Camera.
    let cam = camera::CameraBuilder::new()
        .image_width(1200)
        .ratio(16. / 9.)
        .look_from(na::point![13., 2., 3.])
        .look_at(na::point![0., 0., 0.])
        .view_angle(std::f64::consts::PI / 9.)
        .defocus_angle(std::f64::consts::PI / 180. * 0.6)
        .build();

    // Set World.
    let mut world = vec![
        // Ground
        Entity::new(
            Box::new(Sphere::new(1000., na::point![0., -1000., 0.])),
            Box::new(Lambertian::new(na::vector![0.5, 0.5, 0.5])),
        ),
        Entity::new(
            Box::new(Sphere::new(1., na::point![0., 1., 0.])),
            Box::new(Dielectric::new(na::vector![1., 1., 1.], 1.5)),
        ),
        Entity::new(
            Box::new(Sphere::new(1., na::point![-4., 1., 0.])),
            Box::new(Lambertian::new(na::vector![0.4, 0.2, 0.1])),
        ),
        Entity::new(
            Box::new(Sphere::new(1., na::point![4., 1., 0.])),
            Box::new(Metal::new(na::vector![0.7, 0.6, 0.5], 0.)),
        ),
    ];

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            // Choose material type.
            let choice = rng.gen::<f64>();
            let center = na::point![
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>()
            ];
            if (center - na::point![4., 0.2, 0.]).norm() > 0.9 {
                let material: Box<dyn Material> = if choice < 0.8 {
                    // Lambertian
                    let albedo = na::vector![
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    ];
                    Box::new(Lambertian::new(albedo))
                } else if choice < 0.95 {
                    // Metal
                    let albedo = na::vector![
                        0.5 * (1. + rng.gen::<f64>()),
                        0.5 * (1. + rng.gen::<f64>()),
                        0.5 * (1. + rng.gen::<f64>()),
                    ];
                    let fuzz = 0.5 * rng.gen::<f64>();
                    Box::new(Metal::new(albedo, fuzz))
                } else {
                    // Dielectric
                    Box::new(Dielectric::new(na::vector![1., 1., 1.], 1.5))
                };
                world.push(Entity::new(Box::new(Sphere::new(0.2, center)), material));
            }
        }
    }

    // Render and Show.
    let buffer = cam.render_world(&world);
    let image = utils::into_image(buffer.iter().cloned(), cam.width(), cam.height());
    image.save("image/image.png").expect("Failed to save image");
}
