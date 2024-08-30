use raytracer::vec3::Vec3;
use raytracer::transform::*;

#[test]
fn test_translation_matrix() {
    let delta = Vec3::new(1.0, 2.0, 3.0, true);
    let transform_matrix = translation_matrix(&delta);
    assert_eq!(transform_matrix.mat, [
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 2.0],
        [0.0, 0.0, 1.0, 3.0],
        [0.0, 0.0, 0.0, 1.0]
    ]);
    assert_eq!(transform_matrix.inv, [
        [1.0, 0.0, 0.0, -1.0],
        [0.0, 1.0, 0.0, -2.0],
        [0.0, 0.0, 1.0, -3.0],
        [0.0, 0.0, 0.0,  1.0]
    ]);
}

#[test]
fn test_scaling_matrix() {
    let transform_matrix = scaling_matrix(2.0, 3.0, 4.0);
    assert_eq!(transform_matrix.mat, [
        [2.0, 0.0, 0.0, 0.0],
        [0.0, 3.0, 0.0, 0.0],
        [0.0, 0.0, 4.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]);
    assert_eq!(transform_matrix.inv, [
        [0.5,       0.0,  0.0, 0.0],
        [0.0, 1.0 / 3.0,  0.0, 0.0],
        [0.0,       0.0, 0.25, 0.0],
        [0.0,       0.0,  0.0, 1.0]
    ]);
}

#[test]
fn test_x_rotation_matrix() {
    let theta = 90.0;
    let transform_matrix = x_rotation_matrix(theta);
    let cos_theta = theta.to_radians().cos();
    let sin_theta = theta.to_radians().sin();
    assert_eq!(transform_matrix.mat, [
        [1.0,       0.0,        0.0, 0.0],
        [0.0, cos_theta, -sin_theta, 0.0],
        [0.0, sin_theta,  cos_theta, 0.0],
        [0.0,       0.0,        0.0, 1.0]
    ]);
    assert_eq!(transform_matrix.inv, [
        [1.0,        0.0,       0.0, 0.0],
        [0.0,  cos_theta, sin_theta, 0.0],
        [0.0, -sin_theta, cos_theta, 0.0],
        [0.0,        0.0,       0.0, 1.0]
    ]);
}

#[test]
fn test_y_rotation_matrix() {
    let theta = 90.0;
    let transform_matrix = y_rotation_matrix(theta);
    let cos_theta = theta.to_radians().cos();
    let sin_theta = theta.to_radians().sin();
    assert_eq!(transform_matrix.mat, [
        [ cos_theta,  0.0, sin_theta, 0.0],
        [       0.0,  1.0,       0.0, 0.0],
        [-sin_theta,  0.0, cos_theta, 0.0],
        [       0.0,  0.0,       0.0, 1.0]
    ]);
    assert_eq!(transform_matrix.inv, [
        [cos_theta, 0.0, -sin_theta, 0.0],
        [      0.0, 1.0,        0.0, 0.0],
        [sin_theta, 0.0,  cos_theta, 0.0],
        [      0.0, 0.0,        0.0, 1.0]
    ]);
}

#[test]
fn test_z_rotation_matrix() {
    let theta = 90.0;
    let transform_matrix = z_rotation_matrix(theta);
    let cos_theta = theta.to_radians().cos();
    let sin_theta = theta.to_radians().sin();
    assert_eq!(transform_matrix.mat, [
        [cos_theta, -sin_theta, 0.0, 0.0],
        [sin_theta,  cos_theta, 0.0, 0.0],
        [      0.0,        0.0, 1.0, 0.0],
        [      0.0,        0.0, 0.0, 1.0]
    ]);
    assert_eq!(transform_matrix.inv, [
        [ cos_theta, sin_theta, 0.0, 0.0],
        [-sin_theta, cos_theta, 0.0, 0.0],
        [       0.0,       0.0, 1.0, 0.0],
        [       0.0,       0.0, 0.0, 1.0]
    ]);
}

#[test]
fn test_transform_matrix_multiplication() {
    let transform_matrix1 = translation_matrix(&Vec3::new(1.0, 2.0, 3.0, true));
    let transform_matrix2 = scaling_matrix(2.0, 3.0, 4.0);
    let result = transform_matrix1 * transform_matrix2;
    assert_eq!(result.mat, [
        [2.0, 0.0, 0.0, 1.0],
        [0.0, 3.0, 0.0, 2.0],
        [0.0, 0.0, 4.0, 3.0],
        [0.0, 0.0, 0.0, 1.0]
    ]);
    assert_eq!(result.inv, [
        [0.5,       0.0,  0.0,       -0.5],
        [0.0, 1.0 / 3.0,  0.0, -2.0 / 3.0],
        [0.0,       0.0, 0.25,      -0.75],
        [0.0,       0.0,  0.0,        1.0]
    ]);
}