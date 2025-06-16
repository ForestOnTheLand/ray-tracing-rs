//! Some utility functions for the project.

use nalgebra as na;

/// Generate a unit vector, randomly distributed on S(2).
pub fn random_unit_vector() -> na::Vector3<f64> {
    loop {
        let vector = na::Vector3::from_distribution(
            &rand_distr::Normal::new(0.0, 1.0).unwrap(),
            &mut rand::thread_rng(),
        );
        if !near_zero(vector) {
            return vector.normalize();
        }
    }
}

/// Judge whether the vector is near to zero.
/// The threshold can be adjusted according to need.
pub fn near_zero(vector: na::Vector3<f64>) -> bool {
    vector.data.as_slice().iter().all(|&x| x < 1e-8)
}
