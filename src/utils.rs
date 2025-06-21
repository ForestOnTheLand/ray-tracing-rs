//! Some utility functions for the project.

use nalgebra as na;
use rand::Rng;

/// Generate a unit vector, randomly distributed on S(2).
pub fn random_unit_vector() -> na::UnitVector3<f64> {
    let mut rng = rand::thread_rng();
    let z = rng.gen_range(-1.0..=1.0);
    let r = f64::sqrt(1.0 - z * z);
    let theta = rng.gen_range(0.0..std::f64::consts::TAU);
    na::UnitVector3::new_unchecked(na::vector![r * theta.cos(), r * theta.sin(), z])
}

/// Generate a random point on the unit disk.
pub fn random_in_unit_disk() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let r = f64::sqrt(rng.gen_range(0.0..1.0));
    let theta = rng.gen_range(0.0..std::f64::consts::TAU);
    (r * theta.cos(), r * theta.sin())
}

/// Judge whether the vector is near to zero.
/// The threshold can be adjusted according to need.
#[inline(always)]
pub fn near_zero(vector: na::Vector3<f64>) -> bool {
    vector.data.as_slice().iter().all(|&x| x < 1e-8)
}

/// Convert a buffer of f64 values into an image.
/// The buffer should be arranged as a flattened version of [H, W, 3],
/// with the RGB colors in the range [0., 1.).
pub fn into_image(buffer: impl Iterator<Item = f64>, width: u32, height: u32) -> image::RgbImage {
    let pixels: Vec<u8> = buffer.map(|c| (c.clamp(0., 0.999) * 256.) as u8).collect();
    image::RgbImage::from_raw(width, height, pixels).expect("Failed to create image from buffer")
}
