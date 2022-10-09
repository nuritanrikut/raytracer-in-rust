use std::f64::consts::PI;

pub fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        return min;
    }
    if v > max {
        return max;
    }
    return v;
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}
