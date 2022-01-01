
// Constants
pub static infinity: f64 = f64::INFINITY;
pub static pi: f64 = 3.1415926535897932385;

// Utility Functions
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * (pi / (180.0 as f64));
}
#[inline]
pub fn radians_to_degrees(radians: f64) -> f64 {
    return radians * ((180.0 as f64) / pi);
}

#[inline]
pub fn random_double() -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0..1.0)
}

#[inline]
pub fn random_double_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}