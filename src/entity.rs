//! This module defines all ray tracing related properties of entities in the world.
//! Please refer to [`geometry`] and [`material`] for more details.

/// The geometric property tells us how a ray hits the entity.
mod geometry;
/// The material property tells us how the ray is scattered after hitting the entity.
mod material;

/// Re-export the geometry and material traits and implementations.
pub use self::{
    geometry::{Geometry, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
};

use crate::entity::material::ScatteredRay;
use crate::ray::Ray;

/// An [`Entity`] should consists of geometry and material.
pub struct Entity {
    /// The geometry of the entity, which defines how the ray hits the entity.
    geometry: Box<dyn Geometry>,
    /// The material of the entity, which defines how the ray is scattered after hitting the entity.
    material: Box<dyn Material>,
}

impl Entity {
    /// Create a new [`Entity`] with the given geometry and material.
    pub fn new(geometry: Box<dyn Geometry>, material: Box<dyn Material>) -> Self {
        Self { geometry, material }
    }
}

/// Compute the one-step scattering of a ray on the given entities.
///
/// Note: Currently, I choose to implement this as a function instead of a method.
/// I'll keep this until I find out how this can be generalized into a trait/struct.
pub fn scattering(
    entities: &[Entity],
    ray: &Ray,
    (min_t, mut max_t): (f64, f64),
) -> Option<ScatteredRay> {
    // First, traverse all object and find the nearest object that the ray meets.
    let mut hit_info = None;
    for (i, obj) in entities.iter().enumerate() {
        if let Some(record) = obj.geometry.hit(ray, (min_t, max_t)) {
            max_t = record.t;
            hit_info = Some((i, record));
        }
    }
    let (i, record) = hit_info?;
    // Next, compute scattering on the surface.
    Some(entities[i].material.scatter(ray, &record))
}
