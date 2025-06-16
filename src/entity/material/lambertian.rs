//! Implement the [`Lambertian`] material in 3D space, which models diffuse reflection.

use super::{GeometryHit, Material, Ray, ScatteredRay};
use crate::utils::{near_zero, random_unit_vector};
use nalgebra as na;

/// Diffuse reflection.
pub struct Lambertian {
    /// The attenuation on three color channels.
    albedo: na::Vector3<f64>,
}

impl Lambertian {
    /// Create a new [`Lambertian`] material with the given albedo.
    pub fn new(albedo: na::Vector3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &GeometryHit) -> ScatteredRay {
        let mut scatter_direction = *hit.normal + *random_unit_vector();
        if near_zero(scatter_direction) {
            scatter_direction = *hit.normal;
        }

        ScatteredRay {
            ray: Ray::new(hit.point, scatter_direction),
            decay: self.albedo,
        }
    }
}
