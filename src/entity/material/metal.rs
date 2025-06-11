use nalgebra as na;

use super::{GeometryHit, Material};
use crate::{entity::material::ReflectedRay, ray::Ray};

/// Mirrored reflection.
pub struct Metal {
    albedo: na::Vector3<f64>,
}

impl Metal {
    pub fn new(albedo: na::Vector3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn reflect(&self, ray: &Ray, hit: &GeometryHit) -> ReflectedRay {
        ReflectedRay {
            ray: Ray {
                origin: hit.point,
                direction: ray.direction - 2. * ray.direction.dot(&hit.normal) * hit.normal,
            },
            decay: self.albedo,
        }
    }
}
