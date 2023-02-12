pub fn div_up(a: i32, b: i32) -> i32 {
    (a + (b - 1)) / b
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x < min { min }
    else if x > max { max }
    else { x }
}