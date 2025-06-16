//! This module defines the [`Material`] trait, which should be implemented for
//! a surface material.

/// Implement [`Dielectric`] as a [`Material`].
mod dielectric;
/// Implement [`Lambertian`] as a [`Material`].
mod lambertian;
/// Implement [`Lambertian`] as a [`Material`].
mod metal;

/// Re-export the implemented material types.
pub use self::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

use super::geometry::GeometryHit;
use crate::ray::Ray;
use nalgebra as na;

/// Defines the information of the scattered ray.
pub struct ScatteredRay {
    /// The scattered ray, starting from the incidence point.
    pub ray: Ray,
    /// The decay on the color, for three channels (RGB) respectively. The decay is multiplicative,
    /// i.e. here (0., 0., 0.) means the scattered ray vanishes, and (1., 1., 1.) means the
    /// intensity is unchanged after scattering.
    pub decay: na::Vector3<f64>,
}

/// A trait that computes the scattered ray given the incident ray and the hit information.
pub trait Material {
    /// Compute the scattered ray.
    fn scatter(&self, ray: &Ray, hit: &GeometryHit) -> ScatteredRay;
}

/// Compute the refraction of a ray given the direction and the normal.
///
/// - `eta` is the ri of the incoming medium over the ri of the outgoing medium.
fn refract(
    direction: na::UnitVector3<f64>,
    normal: na::UnitVector3<f64>,
    eta: f64,
) -> Option<na::Vector3<f64>> {
    let cos_theta = -direction.dot(&normal);
    let sin_theta = (1. - cos_theta * cos_theta).sqrt();
    if eta * sin_theta >= 1. {
        return None;
    }
    let r_out_perp = eta * (*direction + cos_theta * *normal);
    let r_out_para = -((1. - r_out_perp.norm_squared()).sqrt()) * *normal;
    Some(r_out_perp + r_out_para)
}

/// Compute the reflection of a ray given the direction and the normal.
fn reflect(direction: na::Vector3<f64>, normal: na::UnitVector3<f64>) -> na::Vector3<f64> {
    direction - 2. * direction.dot(&normal) * normal.as_ref()
}
