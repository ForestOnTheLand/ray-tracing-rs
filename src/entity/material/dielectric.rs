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
        let ri = if hit.exterior { 1. / self.ri } else { self.ri };

        let unit_in = na::UnitVector3::new_normalize(ray.direction);
        let direction = match refract(unit_in, hit.normal, ri) {
            Some(refracted) => {
                // Use Schlick's approximation for reflectance.
                let cosine = -unit_in.dot(&hit.normal).min(1.);
                let r = ((1. - ri) / (1. + ri)).powi(2);
                let reflectance = r + (1. - r) * (1. - cosine).powi(5);
                // Reflect with a certain probability.
                if rand::random::<f64>() < reflectance {
                    reflect(ray.direction, hit.normal)
                } else {
                    refracted
                }
            }
            None => reflect(ray.direction, hit.normal),
        };

        ScatteredRay {
            ray: Ray::new(hit.point, direction),
            decay: self.albedo,
        }
    }
}
