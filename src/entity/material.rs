mod lambertian;
mod metal;

pub use self::{lambertian::Lambertian, metal::Metal};

use nalgebra as na;

use super::geometry::GeometryHit;
use crate::ray::Ray;

/// Defines the information of the reflected ray.
pub struct ReflectedRay {
    /// The reflected ray, starting from the incidence point.
    pub ray: Ray,
    /// The decay on the color, for three channels (RGB) respectively. The decay is multiplicative,
    /// i.e. here (0., 0., 0.) means the reflected ray vanishes, and
    /// (1., 1., 1.) means the intensity is unchanged after reflection.
    pub decay: na::Vector3<f64>,
}

/// This trait should be implemented for materials that can reflect a ray.
/// More specifically, the trait defines how to obtain the reflected ray from an incident ray.
pub trait Material {
    fn reflect(&self, ray: &Ray, hit: &GeometryHit) -> ReflectedRay;
}
