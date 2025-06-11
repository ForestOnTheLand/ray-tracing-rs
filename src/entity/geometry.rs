mod sphere;

pub use self::sphere::Sphere;

use nalgebra as na;

use crate::ray::Ray;

/// Defines the information of the intersection point when a ray hits an visible object.
pub struct GeometryHit {
    /// The intersection point of the ray and the entity.
    pub point: na::Point3<f64>,
    /// The normal vector of the entity surface at the intersection point.
    /// **Note**: The side (inner/outer) matters.
    pub normal: na::Vector3<f64>,
    /// The parameter `t` of the intersection point on the ray.
    pub t: f64,
}

/// This trait should be implemented for an entity that is visible.
/// More specifically, the trait defines how should the objects interact with a ray.
pub trait Geometry {
    /// Compute the intersection of the entity itself and the ray defined by `ray`, whose
    /// parameter `t` lies in `t_range`.
    ///
    /// Returns the intersection information if any.
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<GeometryHit>;
}
