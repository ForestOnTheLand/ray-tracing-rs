//! This module defines the [`Material`] trait, which should be implemented for
//! a surface material.

/// Implement [`Lambertian`] as a [`Material`].
mod lambertian;
/// Implement [`Lambertian`] as a [`Material`].
mod metal;

/// Re-export the implemented material types.
pub use self::{lambertian::Lambertian, metal::Metal};

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
