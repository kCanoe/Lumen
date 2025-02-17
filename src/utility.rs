use rand::Rng;
use rand::distributions::{Distribution, Uniform};

const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(theta: f64) ->  f64 {
    theta * PI / 180.0
}

pub fn random() -> f64 {
    let range = Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    range.sample(&mut rng)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let range = Uniform::new(min, max);
    let mut rng = rand::thread_rng();
    range.sample(&mut rng)
}
