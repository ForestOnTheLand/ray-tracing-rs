use image::Rgb;
use nalgebra as na;

use crate::entity::Entities;
use crate::ray::Ray;
use crate::utils::near_zero;

/// Defines the configuration of the world camera.
#[allow(dead_code)]
pub struct Camera {
    /// Entityidth of output image, in pixels.
    image_width: u32,
    /// Height of output image, in pixels.
    image_height: u32,
    /// The projected distance between two horizontally adjacent pixels.
    /// Direction: Left to Right.
    pixel_du: na::Vector3<f64>,
    /// The projected distance between two vertically adjacent pixels.
    /// Direction: Up to Down.
    pixel_dv: na::Vector3<f64>,
    /// Ratio of width over height. `ratio = image_width / image_height = pixel_du / pixel_dv`.
    ratio: f64,
    /// Camera center.
    center: na::Point3<f64>,
    /// The projected location of the center of pixel at (0, 0).
    base_pixel_loc: na::Point3<f64>,
}

impl Camera {
    /// Maximum number of reflections before the ray disappears.
    const MAX_REFLECTION: i32 = 50;
    /// Number of rays sampled per pixel.
    const SAMPLING: i32 = 10;
}

impl Camera {
    /// Create a camera by the width (see [`Camera::image_width`]) and
    /// ratio (see [`Camera::ratio`]).
    ///
    /// **TODO**: Make it more flexible!
    pub fn from_width_height(image_width: u32, image_height: u32) -> Self {
        let ratio = image_width as f64 / image_height as f64;

        // Currently, we fix those parameters. More flexibility will be provided.
        let focal_length = 1.0;
        let proj_height = 2.0;
        let proj_width = proj_height * ratio;
        let center = na::point![0., 0., 0.];

        let proj_u = na::vector![proj_width, 0., 0.];
        let proj_v = na::vector![0., -proj_height, 0.];

        let pixel_du = proj_u / image_width as f64;
        let pixel_dv = proj_v / image_height as f64;

        // Currently, the negative z-axis of the camera points to the center of the image.
        let base_pixel_loc = center - na::vector![0., 0., focal_length] - proj_u / 2. - proj_v / 2.;

        Self {
            image_width,
            image_height,
            pixel_du,
            pixel_dv,
            ratio,
            center,
            base_pixel_loc,
        }
    }

    /// Render a ray which interacts with given objects.
    fn render_ray(ray: Ray, objects: &Entities) -> na::Vector3<f64> {
        fn render_ray_impl(ray: Ray, objects: &Entities, reflect: i32) -> na::Vector3<f64> {
            // Quit if reflects too much times.
            if reflect >= Camera::MAX_REFLECTION {
                return na::vector![0., 0., 0.];
            }

            // Foreground objects.
            if let Some(ray) = objects.forward_ray(&ray, (0.001, f64::INFINITY)) {
                if near_zero(ray.decay) {
                    return na::vector![0., 0., 0.];
                }
                return render_ray_impl(ray.ray, objects, reflect + 1).component_mul(&ray.decay);
            }

            // Background
            let unit = ray.direction.normalize();
            let alpha = 0.5 * (unit.y + 1.0);
            (1. - alpha) * na::vector![1., 1., 1.] + alpha * na::vector![0.5, 0.7, 1.0]
        }

        render_ray_impl(ray, objects, 0)
    }

    /// Render whole image with given objects.
    pub fn render_world(&self, objects: &Entities) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut image_buf = image::ImageBuffer::new(self.image_width, self.image_height);
        for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
            let mut color = na::vector![0., 0., 0.];
            for _ in 0..Self::SAMPLING {
                let delta_x = rand::random::<f64>() - 0.5;
                let delta_y = rand::random::<f64>() - 0.5;
                let ray = Ray {
                    origin: self.center,
                    direction: self.base_pixel_loc
                        + (x as f64 + delta_x) * self.pixel_du
                        + (y as f64 + delta_y) * self.pixel_dv
                        - self.center,
                };
                color += Self::render_ray(ray, objects);
            }
            *pixel = to_rgb(color.unscale(Self::SAMPLING as f64));
        }
        image_buf
    }
}

/// Convert RGB in [`f64`] (from 0.0 to 1.0) into [`u8`] (from 0 to 255).
fn to_rgb(color: na::Vector3<f64>) -> image::Rgb<u8> {
    let [[r, g, b]] = color.data.0;
    image::Rgb([
        (r.clamp(0., 0.999) * 256.) as u8,
        (g.clamp(0., 0.999) * 256.) as u8,
        (b.clamp(0., 0.999) * 256.) as u8,
    ])
}
