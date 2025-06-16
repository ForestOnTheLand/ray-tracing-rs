//! Implement the [`Metal`] material in 3D space, which models mirrored reflection.

use super::{reflect, GeometryHit, Material, Ray, ScatteredRay};
use crate::utils::random_unit_vector;
use nalgebra as na;

/// Mirrored reflection.
pub struct Metal {
    /// The attenuation on three color channels.
    albedo: na::Vector3<f64>,
    /// The randomness of the reflection, which is used to simulate roughness.
    fuzz: f64,
}

impl Metal {
    /// Create a new [`Metal`] material with the given albedo.
    pub fn new(albedo: na::Vector3<f64>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &GeometryHit) -> ScatteredRay {
        ScatteredRay {
            ray: Ray::new(
                hit.point,
                reflect(ray.direction, hit.normal) + self.fuzz * *random_unit_vector(),
            ),
            decay: self.albedo,
        }
    }
}
