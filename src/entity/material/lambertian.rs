use nalgebra as na;

use super::{GeometryHit, Material};
use crate::entity::material::ReflectedRay;
use crate::ray::Ray;
use crate::utils::{near_zero, random_unit_vector};

/// Diffuse reflection.
pub struct Lambertian {
    /// The attenuation on three color channels.
    albedo: na::Vector3<f64>,
}

impl Lambertian {
    pub fn new(albedo: na::Vector3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn reflect(&self, _ray: &Ray, hit: &GeometryHit) -> ReflectedRay {
        let scatter_direction = hit.normal + random_unit_vector();

        ReflectedRay {
            ray: Ray {
                origin: hit.point,
                direction: (if near_zero(scatter_direction) {
                    hit.normal
                } else {
                    scatter_direction
                }),
            },
            decay: self.albedo,
        }
    }
}
