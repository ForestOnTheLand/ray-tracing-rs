//! Implement the [`Metal`] material in 3D space, which models mirrored reflection.

use super::{GeometryHit, Material, Ray, ScatteredRay};
use nalgebra as na;

/// Mirrored reflection.
pub struct Metal {
    albedo: na::Vector3<f64>,
}

impl Metal {
    /// Create a new [`Metal`] material with the given albedo.
    pub fn new(albedo: na::Vector3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &GeometryHit) -> ScatteredRay {
        ScatteredRay {
            ray: Ray {
                origin: hit.point,
                direction: ray.direction - 2. * ray.direction.dot(&hit.normal) * hit.normal,
            },
            decay: self.albedo,
        }
    }
}
