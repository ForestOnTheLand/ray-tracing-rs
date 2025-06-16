//! Implement the [`Dielectric`] material in 3D space, which models refraction and reflection.

use super::{reflect, refract, GeometryHit, Material, Ray, ScatteredRay};
use nalgebra as na;

/// Refraction and reflection.
pub struct Dielectric {
    /// The attenuation on three color channels.
    albedo: na::Vector3<f64>,
    /// The refractive index of the material, with respect to the exterior medium.
    ri: f64,
}

impl Dielectric {
    /// Create a new [`Dielectric`] material with the given albedo and refractive index.
    pub fn new(albedo: na::Vector3<f64>, ri: f64) -> Self {
        Self { albedo, ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &GeometryHit) -> ScatteredRay {
        let ri = if hit.exterior { 1.0 / self.ri } else { self.ri };

        let unit_direction = na::UnitVector3::new_normalize(ray.direction);
        let direction = match refract(unit_direction, hit.normal, ri) {
            Some(refracted) => {
                let cosine = -unit_direction.dot(&hit.normal).min(1.0);
                if rand::random::<f64>() < reflectance(cosine, ri) {
                    reflect(ray.direction, hit.normal)
                } else {
                    refracted
                }
            }
            None => reflect(ray.direction, hit.normal),
        };

        ScatteredRay {
            ray: Ray {
                origin: hit.point,
                direction,
            },
            decay: self.albedo,
        }
    }
}

/// Calculate reflectance using Schlick's approximation.
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1. - refraction_index) / (1. + refraction_index)).powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
