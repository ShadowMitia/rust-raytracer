use rand::prelude::*;

use crate::maths::vec3::*;

pub fn random_in_unit_sphere() -> Vec3 {
    let a = random_between(0.0, 2.0 * std::f32::consts::PI);
    let z = random_between(-1.0, 1.0);
    let r = f32::sqrt(1.0 - z * z);

    Vec3::new(r * f32::cos(a), r * f32::sin(a), z)
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn random_01() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_between(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
