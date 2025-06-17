//! Some utility functions for the project.

use nalgebra as na;

/// Generate a unit vector, randomly distributed on S(2).
pub fn random_unit_vector() -> na::UnitVector3<f64> {
    loop {
        let vector = na::Vector3::from_distribution(
            &rand_distr::Normal::new(0., 1.).unwrap(),
            &mut rand::thread_rng(),
        );
        if let Some(unit_vector) = na::UnitVector3::try_new(vector, 1e-8) {
            return unit_vector;
        }
    }
}

/// Generate a random point on the unit disk.
pub fn random_in_unit_disk() -> (f64, f64) {
    loop {
        let x = rand::random::<f64>() - 0.5;
        let y = rand::random::<f64>() - 0.5;
        if x * x + y * y < 1. {
            return (x, y);
        }
    }
}

/// Judge whether the vector is near to zero.
/// The threshold can be adjusted according to need.
#[inline(always)]
pub fn near_zero(vector: na::Vector3<f64>) -> bool {
    vector.data.as_slice().iter().all(|&x| x < 1e-8)
}
