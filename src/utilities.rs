static PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    return x;
}
