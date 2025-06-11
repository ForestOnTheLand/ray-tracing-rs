use nalgebra as na;

use crate::ray::Ray;

/// Defines the information of the intersection point when a ray hits an visible object.
pub struct HitRecord {
    pub point: na::Point3<f64>,
    pub normal: na::Vector3<f64>,
    pub t: f64,
}

/// This trait should be implemented for an entity that is visible.
/// More specifically, the trait defines how should the objects interact with a ray.
pub trait Visible {
    /// Compute the intersection of the entity itself and the ray defined by `ray`, whose
    /// parameter `t` lies in `t_range`.
    ///
    /// Returns the intersection information if any.
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord>;
}

impl Visible for Vec<Box<dyn Visible>> {
    fn hit(&self, ray: &Ray, (min_t, max_t): (f64, f64)) -> Option<HitRecord> {
        let mut best_record = None;
        let mut max_t = max_t;
        for obj in self.iter() {
            if let Some(record) = obj.hit(ray, (min_t, max_t)) {
                max_t = record.t;
                best_record = Some(record);
            }
        }
        best_record
    }
}
