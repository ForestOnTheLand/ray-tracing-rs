//! This module defines the [`Geometry`] trait, which should be implemented for
//! a geometry shape.

/// Implement [`Sphere`] as a [`Geometry`].
mod sphere;

/// Re-export the implemented geometry shapes.
pub use self::sphere::Sphere;

use crate::ray::Ray;
use nalgebra as na;

/// Defines the information of the intersection point when a ray hits an visible object.
pub struct GeometryHit {
    /// The intersection point of the ray and the entity.
    pub point: na::Point3<f64>,
    /// The normal vector of the entity surface at the intersection point.
    pub normal: na::Vector3<f64>,
    /// The orientation of the normal vector.
    #[allow(dead_code)]
    pub exterior: bool,
    /// The parameter `t` of the intersection point on the ray.
    pub t: f64,
}

impl GeometryHit {
    /// Create a new [`GeometryHit`] instance from ray, t and normal.
    fn new(ray: &Ray, normal: na::Vector3<f64>, t: f64) -> Self {
        let point = ray.at(t);
        let exterior = normal.dot(&ray.direction) < 0.;
        let normal = if exterior { normal } else { -normal };
        Self {
            point,
            normal,
            exterior,
            t,
        }
    }
}

/// A trait that computes the intersection of a ray and a geometry shape.
pub trait Geometry {
    /// Compute the intersection of the ray (with a specified range) and the geometry.
    ///
    /// Returns `None` if the ray does not hit the geometry within the specified range.    
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<GeometryHit>;
}
