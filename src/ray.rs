//! Defines a [`Ray`] in 3D space.

use nalgebra as na;

/// A ray in the 3D space, determined by the origin point and its direction.
#[derive(Debug, Clone)]
pub struct Ray {
    /// The source point of the ray.
    pub origin: na::Point3<f64>,
    /// The direction of the ray. Note that the direction vector is not necessarily a unit vector.
    pub direction: na::Vector3<f64>,
}

impl Ray {
    /// The ray is parameterized by a parameter `t`, and this returns the point at position `t`.
    pub fn at(&self, t: f64) -> na::Point3<f64> {
        self.origin + t * self.direction
    }
}
