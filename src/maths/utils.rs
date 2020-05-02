use rand::prelude::*;

use crate::maths::vec3::*;

pub fn random_in_unit_sphere() -> Vec3 {
    let a = random_between(0.0, 2.0 * std::f64::consts::PI);
    let z = random_between(-1.0, 1.0);
    let r = f64::sqrt(1.0 - z * z);

    Vec3::new(r * f64::cos(a), r * f64::sin(a), z)
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_01() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_between(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
