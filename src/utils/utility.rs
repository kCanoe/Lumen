//use rand::Rng;
use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};

const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(theta: f64) ->  f64 {
    theta * PI / 180.0
}

pub fn random(dist: &Uniform<f64>, rng: &mut ThreadRng) -> f64 {
    dist.sample(rng)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let range = Uniform::new(min, max);
    let mut rng = rand::thread_rng();
    range.sample(&mut rng)
}
