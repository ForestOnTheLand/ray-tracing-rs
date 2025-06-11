mod geometry;
mod material;

pub use self::geometry::{Geometry, GeometryHit, Sphere};
pub use self::material::{Lambertian, Material, Metal};

use crate::entity::material::ReflectedRay;
use crate::ray::Ray;

/// An [`Entity`] should consists of material and geometry.
pub struct Entity {
    material: Box<dyn Material>,
    geometry: Box<dyn Geometry>,
}

impl Entity {
    pub fn new(material: Box<dyn Material>, geometry: Box<dyn Geometry>) -> Self {
        Self { material, geometry }
    }
}

impl Material for Entity {
    fn reflect(&self, ray: &Ray, hit: &GeometryHit) -> ReflectedRay {
        self.material.reflect(ray, hit)
    }
}

impl Geometry for Entity {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<GeometryHit> {
        self.geometry.hit(ray, t_range)
    }
}

/// A list of [`Entity`]
pub struct Entities(Vec<Entity>);

impl Entities {
    pub fn new(entities: Vec<Entity>) -> Self {
        Self(entities)
    }
}

impl Entities {
    pub fn forward_ray(&self, ray: &Ray, (min_t, max_t): (f64, f64)) -> Option<ReflectedRay> {
        // First, traverse all object and find the nearest object that the ray meets.
        let mut hit_info = None;
        let mut max_t = max_t;
        for (i, obj) in self.0.iter().enumerate() {
            if let Some(record) = obj.hit(ray, (min_t, max_t)) {
                max_t = record.t;
                hit_info = Some((i, record));
            }
        }
        let (i, record) = hit_info?;
        // Next, compute reflection on the surface.
        Some(self.0[i].reflect(ray, &record))
    }
}
