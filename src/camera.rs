//! Defines [`Camera`] that renders the world.

use crate::entity::{scattering, Entity};
use crate::ray::Ray;
use crate::utils::{near_zero, random_in_unit_disk};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use nalgebra as na;
use rayon::prelude::*;

pub struct CameraBuilder {
    // Note: Exactly 2 fields in `image_width`, `image_height`, and `ratio` should be set.
    image_width: Option<u32>,
    image_height: Option<u32>,
    ratio: Option<f64>,
    // The following fields have default values.
    look_from: na::Point3<f64>,
    look_at: na::Point3<f64>,
    up: na::Vector3<f64>,
    view_angle: f64,
    focal_dist: f64,
    defocus_angle: f64,
}

impl CameraBuilder {
    /// Create a uninitialized [`CameraBuilder`].
    pub fn new() -> Self {
        Self {
            image_width: None,
            image_height: None,
            ratio: None,
            look_from: na::point![0., 0., 0.],
            look_at: na::point![0., 0., -1.],
            up: na::vector![0., 1., 0.],
            view_angle: std::f64::consts::FRAC_PI_2,
            focal_dist: 10.,
            defocus_angle: 0.,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraBuilder {
    /// Set width.
    pub fn image_width(mut self, width: u32) -> Self {
        self.image_width = Some(width);
        self
    }

    /// Set height.
    pub fn image_height(mut self, height: u32) -> Self {
        self.image_height = Some(height);
        self
    }

    /// Set ratio.
    pub fn ratio(mut self, ratio: f64) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// Set focal distance.
    pub fn focal_dist(mut self, dist: f64) -> Self {
        self.focal_dist = dist;
        self
    }

    /// Set defocus angle.
    pub fn defocus_angle(mut self, angle: f64) -> Self {
        self.defocus_angle = angle;
        self
    }

    /// Set look from point.
    pub fn look_from(mut self, point: na::Point3<f64>) -> Self {
        self.look_from = point;
        self
    }

    /// Set look at point.
    pub fn look_at(mut self, point: na::Point3<f64>) -> Self {
        self.look_at = point;
        self
    }

    /// Set up vector.
    pub fn up(mut self, up: na::Vector3<f64>) -> Self {
        self.up = up;
        self
    }

    /// Set view angle.
    pub fn view_angle(mut self, angle: f64) -> Self {
        self.view_angle = angle;
        self
    }

    /// Build a [`Camera`] with the current configuration.
    pub fn build(self) -> Camera {
        // Get image size options.
        let (image_width, image_height) = match (self.image_width, self.image_height, self.ratio) {
            (Some(w), Some(h), _) => (w, h),
            (None, Some(h), Some(r)) => ((h as f64 * r).round() as u32, h),
            (Some(w), None, Some(r)) => (w, (w as f64 / r).round() as u32),
            _ => {
                panic!("CameraBuilder: at least two of `image_width`, `image_height`, and `ratio` should be set.")
            }
        };
        let ratio = image_width as f64 / image_height as f64;

        // Compute orthonormal axes of camera.
        let w_axis = (self.look_from - self.look_at).normalize();
        let u_axis = self.up.cross(&w_axis).normalize();
        let v_axis = w_axis.cross(&u_axis).normalize();

        // Compute focal length and viewport size according to the view angle and look from/to points.
        let viewport_height = 2. * self.focal_dist * (self.view_angle / 2.).tan();
        let viewport_width = viewport_height * ratio;

        let viewport_u = viewport_width * u_axis;
        let viewport_v = viewport_height * -v_axis;

        let base_pixel_loc =
            self.look_from - self.focal_dist * w_axis - viewport_u / 2. - viewport_v / 2.;

        let defocus_radius = (self.defocus_angle / 2.).tan() * self.focal_dist;

        Camera {
            image_width,
            image_height,
            pixel_du: viewport_u / image_width as f64,
            pixel_dv: viewport_v / image_height as f64,
            center: self.look_from,
            base_pixel_loc,
            defocus_u: defocus_radius * u_axis,
            defocus_v: defocus_radius * v_axis,
        }
    }
}

/// Defines the configuration of the world camera.
/// You should use [`CameraBuilder`] to build a [`Camera`].
pub struct Camera {
    /// Width of output image, in pixels.
    image_width: u32,
    /// Height of output image, in pixels.
    image_height: u32,
    /// The viewport distance between two horizontally adjacent pixels.
    /// Direction: Left to Right.
    pixel_du: na::Vector3<f64>,
    /// The viewport distance between two vertically adjacent pixels.
    /// Direction: Up to Down.
    pixel_dv: na::Vector3<f64>,
    /// Camera center. This is also known as the "look from" point.
    center: na::Point3<f64>,
    /// The viewport location of the left-top corner of the image.
    base_pixel_loc: na::Point3<f64>,
    defocus_u: na::Vector3<f64>,
    defocus_v: na::Vector3<f64>,
}

impl Camera {
    /// Maximum number of scatters before the ray disappears.
    const MAX_SCATTER: i32 = 50;
    /// Number of rays sampled per pixel.
    const SAMPLING: i32 = 500;
    /// Style of the progress bar.
    const PB_STYLE: &'static str =
        "Rendering: {wide_bar:.green/yellow} {pos:>7}/{len:7} {elapsed_precise}/{duration_precise}";
}

impl Camera {
    /// Obtain width of the output image.
    pub fn width(&self) -> u32 {
        self.image_width
    }

    /// Obtain height of the output image.
    pub fn height(&self) -> u32 {
        self.image_height
    }
}

impl Camera {
    /// Render a ray which interacts with given objects.
    fn render_ray(ray: Ray, objects: &[Entity]) -> na::Vector3<f64> {
        // Inner implementation.
        fn render_ray_impl(ray: Ray, objects: &[Entity], scatter: i32) -> na::Vector3<f64> {
            // Quit if scatters too much times.
            if scatter >= Camera::MAX_SCATTER {
                return na::vector![0., 0., 0.];
            }

            // Foreground objects.
            if let Some(ray) = scattering(objects, &ray, (0.001, f64::INFINITY)) {
                if near_zero(ray.decay) {
                    return na::vector![0., 0., 0.];
                }
                return render_ray_impl(ray.ray, objects, scatter + 1).component_mul(&ray.decay);
            }

            // Background
            let alpha = 0.5 * (ray.direction.normalize().y + 1.);
            (1. - alpha) * na::vector![1., 1., 1.] + alpha * na::vector![0.5, 0.7, 1.]
        }

        render_ray_impl(ray, objects, 0)
    }

    /// Sample a ray to render the given pixel.
    /// The ray should start from the camera center and point to the pixel.
    fn sample_ray(&self, x: u32, y: u32) -> Ray {
        let (delta_x, delta_y) = random_in_unit_disk();
        let source = self.center + delta_x * self.defocus_u + delta_y * self.defocus_v;
        let target = self.base_pixel_loc
            + (x as f64 + rand::random::<f64>()) * self.pixel_du
            + (y as f64 + rand::random::<f64>()) * self.pixel_dv;
        Ray {
            origin: source,
            direction: target - source,
        }
    }
}

impl Camera {
    /// Render whole image with given objects.
    ///
    /// This function only render one pass, and return a flattened vector of shape [H, W, 3],
    /// where each element is in [0.0, 1.0) and each pixel is RGB format.
    pub fn render_world_single(&self, objects: &[Entity], pb: ProgressBar) -> na::DVector<f64> {
        let tid = rayon::current_thread_index().unwrap();
        core_affinity::set_for_current(core_affinity::CoreId { id: tid });

        let mut image_buf = Vec::with_capacity((self.image_width * self.image_height * 3) as usize);
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let color = Self::render_ray(self.sample_ray(x, y), objects);
                image_buf.extend_from_slice(color.as_slice());
                pb.inc(1);
            }
        }
        pb.finish();
        na::DVector::from_vec(image_buf)
    }

    /// Render whole image with given objects.
    pub fn render_world(&self, objects: &[Entity]) -> na::DVector<f64> {
        let mpb = MultiProgress::new();
        let style = ProgressStyle::with_template(Self::PB_STYLE).unwrap();

        (0..Self::SAMPLING)
            .into_par_iter()
            .map(|i| {
                let pb = ProgressBar::new((self.image_width * self.image_height) as u64)
                    .with_style(style.clone())
                    .with_prefix(i.to_string());
                self.render_world_single(objects, mpb.add(pb))
            })
            .reduce(
                || na::DVector::zeros((self.image_width * self.image_height * 3) as usize),
                std::ops::Add::add,
            )
            .unscale(Self::SAMPLING as f64)
    }
}
