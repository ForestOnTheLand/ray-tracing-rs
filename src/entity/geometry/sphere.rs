//! Implement a [`Sphere`] in 3D space.

use super::{Geometry, GeometryHit};
use crate::ray::Ray;
use nalgebra as na;

/// A sphere in 3D space which is parameterized by its radius and center.
pub struct Sphere {
    /// The radius of the sphere.
    pub radius: f64,
    /// The center of the sphere in 3D space.
    pub center: na::Point3<f64>,
}

impl Sphere {
    /// Create a sphere in 3D space with given radius and center.
    pub fn new(radius: f64, center: na::Point3<f64>) -> Self {
        Self { radius, center }
    }
}

impl Geometry for Sphere {
    fn hit(&self, ray: &Ray, (min_t, max_t): (f64, f64)) -> Option<GeometryHit> {
        let oc = self.center - ray.origin;
        let a = ray.direction.norm_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.norm_squared() - self.radius.powi(2);
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        let mut t = (h - discriminant.sqrt()) / a;

        if t <= min_t || max_t <= t {
            t = (h + discriminant.sqrt()) / a;
            if t <= min_t || max_t <= t {
                return None;
            }
        }

        let point = ray.at(t);
        let normal = (point - self.center).normalize();
        Some(GeometryHit::new(ray, normal, t))
    }
}
