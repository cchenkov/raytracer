pub fn div_up(a: i32, b: i32) -> i32 {
    (a + (b - 1)) / b
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x < min { min }
    else if x > max { max }
    else { x }
}

pub fn transpose(matrix: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut result = [[0.0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            result[j][i] = matrix[i][j];
        }
    }

    result
}