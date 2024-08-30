use raytracer::ray::Ray;
use raytracer::vec3::Vec3;
use raytracer::transform::*;

const EPSILON: f64 = 1e-10;

fn approx_eq(a: Vec3, b: Vec3) -> bool {
    (a.x() - b.x()).abs() < EPSILON &&
    (a.y() - b.y()).abs() < EPSILON &&
    (a.z() - b.z()).abs() < EPSILON &&
    a.is_point() == b.is_point()
}

#[test]
fn test_ray_new() {
    let origin = Vec3::new(1.0, 2.0, 3.0, true);
    let direction = Vec3::new(4.0, 5.0, 6.0, false);
    let ray = Ray::new(origin, direction);
    assert_eq!(ray.origin(), origin);
    assert_eq!(ray.direction(), direction);
    assert_eq!(ray.direction_inv(), Vec3::new(1.0 / 4.0, 1.0 / 5.0, 1.0 / 6.0, false));
}

#[test]
fn test_ray_at() {
    let origin = Vec3::new(1.0, 2.0, 3.0, true);
    let direction = Vec3::new(4.0, 5.0, 6.0, false);
    let ray = Ray::new(origin, direction);
    let t = 2.0;
    assert_eq!(ray.at(t), Vec3::new(9.0, 12.0, 15.0, true));
}

#[test]
fn test_ray_transform_translation() {
    let origin = Vec3::new(1.0, 2.0, 3.0, true);
    let direction = Vec3::new(4.0, 5.0, 6.0, false);
    let ray = Ray::new(origin, direction);
    let translation = translation_matrix(&Vec3::new(1.0, 2.0, 3.0, true));
    let transformed_ray = ray.transform(&translation.mat);
    assert_eq!(transformed_ray.origin(), Vec3::new(2.0, 4.0, 6.0, true));
    assert_eq!(transformed_ray.direction(), direction);
}

#[test]
fn test_ray_transform_scaling() {
    let origin = Vec3::new(1.0, 2.0, 3.0, true);
    let direction = Vec3::new(4.0, 5.0, 6.0, false);
    let ray = Ray::new(origin, direction);
    let scaling = scaling_matrix(2.0, 3.0, 4.0);
    let transformed_ray = ray.transform(&scaling.mat);
    assert_eq!(transformed_ray.origin(), Vec3::new(2.0, 6.0, 12.0, true));
    assert_eq!(transformed_ray.direction(), Vec3::new(8.0, 15.0, 24.0, false));
}

#[test]
fn test_ray_transform_x_rotation() {
    let origin = Vec3::new(1.0, 2.0, 3.0, true);
    let direction = Vec3::new(4.0, 5.0, 6.0, false);
    let ray = Ray::new(origin, direction);
    let rotation = x_rotation_matrix(90.0);
    let transformed_ray = ray.transform(&rotation.mat);
    let expected_origin = Vec3::new(1.0, -3.0, 2.0, true);
    let expected_direction = Vec3::new(4.0, -6.0, 5.0, false);
    assert!(approx_eq(transformed_ray.origin(), expected_origin));
    assert!(approx_eq(transformed_ray.direction(), expected_direction));
}

#[test]
fn test_ray_transform_y_rotation() {
    let origin = Vec3::new(1.0, 2.0, 3.0, true);
    let direction = Vec3::new(4.0, 5.0, 6.0, false);
    let ray = Ray::new(origin, direction);
    let rotation = y_rotation_matrix(90.0);
    let transformed_ray = ray.transform(&rotation.mat);
    let expected_origin = Vec3::new(3.0, 2.0, -1.0, true);
    let expected_direction = Vec3::new(6.0, 5.0, -4.0, false);
    assert!(approx_eq(transformed_ray.origin(), expected_origin));
    assert!(approx_eq(transformed_ray.direction(), expected_direction));
}

#[test]
fn test_ray_transform_z_rotation() {
    let origin = Vec3::new(1.0, 2.0, 3.0, true);
    let direction = Vec3::new(4.0, 5.0, 6.0, false);
    let ray = Ray::new(origin, direction);
    let rotation = z_rotation_matrix(90.0);
    let transformed_ray = ray.transform(&rotation.mat);
    let expected_origin = Vec3::new(-2.0, 1.0, 3.0, true);
    let expected_direction = Vec3::new(-5.0, 4.0, 6.0, false);
    assert!(approx_eq(transformed_ray.origin(), expected_origin));
    assert!(approx_eq(transformed_ray.direction(), expected_direction));
}