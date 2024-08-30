use raytracer::vec3::Vec3;
use raytracer::transform::Transform;


#[test]
fn test_vec3_new() {
    let v = Vec3::new(1.0, 2.0, 3.0, true);
    assert_eq!(v.x(), 1.0);
    assert_eq!(v.y(), 2.0);
    assert_eq!(v.z(), 3.0);
    assert!(v.is_point());
}

#[test]
fn test_vec3_length() {
    let v = Vec3::new(1.0, 2.0, 2.0, false);
    assert_eq!(v.length(), 3.0);
}

#[test]
fn test_vec3_normalized() {
    let v = Vec3::new(1.0, 2.0, 2.0, false);
    let normalized = v.normalized();
    assert!((normalized.length() - 1.0).abs() < 1e-6);
}

#[test]
fn test_vec3_dot() {
    let v1 = Vec3::new(1.0, 2.0, 3.0, false);
    let v2 = Vec3::new(4.0, -5.0, 6.0, false);
    assert_eq!(v1.dot(v2), 12.0);
}

#[test]
fn test_vec3_cross() {
    let v1 = Vec3::new(1.0, 2.0, 3.0, false);
    let v2 = Vec3::new(4.0, 5.0, 6.0, false);
    let cross = v1.cross(v2);
    assert_eq!(cross, Vec3::new(-3.0, 6.0, -3.0, false));
}

#[test]
fn test_vec3_reflect() {
    let v = Vec3::new(1.0, -1.0, 0.0, false);
    let normal = Vec3::new(0.0, 1.0, 0.0, false);
    let reflected = v.reflect(normal);
    assert_eq!(reflected, Vec3::new(1.0, 1.0, 0.0, false));
}

#[test]
fn test_vec3_add() {
    let v1 = Vec3::new(1.0, 2.0, 3.0, false);
    let v2 = Vec3::new(4.0, 5.0, 6.0, false);
    assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0, false));
}

#[test]
fn test_vec3_add_scalar() {
    let v = Vec3::new(1.0, 2.0, 3.0, false);
    assert_eq!(v + 1.0, Vec3::new(2.0, 3.0, 4.0, false));
}

#[test]
fn test_scalar_add_vec3() {
    let v = Vec3::new(1.0, 2.0, 3.0, false);
    assert_eq!(1.0 + v, Vec3::new(2.0, 3.0, 4.0, false));
}

#[test]
fn test_vec3_sub() {
    let v1 = Vec3::new(1.0, 2.0, 3.0, false);
    let v2 = Vec3::new(4.0, 5.0, 6.0, false);
    assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0, false));
}

#[test]
fn test_vec3_sub_scalar() {
    let v = Vec3::new(1.0, 2.0, 3.0, false);
    assert_eq!(v - 1.0, Vec3::new(0.0, 1.0, 2.0, false));
}

#[test]
fn test_scalar_sub_vec3() {
    let v = Vec3::new(1.0, 2.0, 3.0, false);
    assert_eq!(1.0 - v, Vec3::new(0.0, -1.0, -2.0, false));
}

#[test]
fn test_vec3_mul() {
    let v1 = Vec3::new(1.0, 2.0, 3.0, false);
    let v2 = Vec3::new(4.0, 5.0, 6.0, false);
    assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0, false));
}

#[test]
fn test_vec3_mul_scalar() {
    let v = Vec3::new(1.0, 2.0, 3.0, false);
    assert_eq!(v * 2.0, Vec3::new(2.0, 4.0, 6.0, false));
}

#[test]
fn test_scalar_mul_vec3() {
    let v = Vec3::new(1.0, 2.0, 3.0, false);
    assert_eq!(2.0 * v, Vec3::new(2.0, 4.0, 6.0, false));
}

#[test]
fn test_vec3_div() {
    let v1 = Vec3::new(4.0, 6.0, 8.0, false);
    let v2 = Vec3::new(2.0, 3.0, 4.0, false);
    assert_eq!(v1 / v2, Vec3::new(2.0, 2.0, 2.0, false));
}

#[test]
fn test_vec3_div_scalar() {
    let v = Vec3::new(4.0, 6.0, 8.0, false);
    assert_eq!(v / 2.0, Vec3::new(2.0, 3.0, 4.0, false));
}

#[test]
fn test_scalar_div_vec3() {
    let v = Vec3::new(2.0, 4.0, 8.0, false);
    assert_eq!(16.0 / v, Vec3::new(8.0, 4.0, 2.0, false));
}

#[test]
fn test_vec3_neg() {
    let v = Vec3::new(1.0, -2.0, 3.0, false);
    assert_eq!(-v, Vec3::new(-1.0, 2.0, -3.0, false));
}

#[test]
fn test_vec3_index() {
    let v = Vec3::new(1.0, 2.0, 3.0, false);
    assert_eq!(v[0], 1.0);
    assert_eq!(v[1], 2.0);
    assert_eq!(v[2], 3.0);
}

#[test]
#[should_panic(expected = "Index out of range")]
fn test_vec3_index_out_of_range() {
    let v = Vec3::new(1.0, 2.0, 3.0, false);
    let _ = v[3];
}

#[test]
fn test_vec3_transform_point() {
    let v = Vec3::new(1.0, 2.0, 3.0, true);
    let matrix = [
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 2.0],
        [0.0, 0.0, 1.0, 3.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let transformed = v.transform(&matrix);
    assert_eq!(transformed, Vec3::new(2.0, 4.0, 6.0, true));
}

#[test]
fn test_vec3_transform_vector() {
    let v = Vec3::new(1.0, 2.0, 3.0, false);
    let matrix = [
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 2.0],
        [0.0, 0.0, 1.0, 3.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let transformed = v.transform(&matrix);
    assert_eq!(transformed, Vec3::new(1.0, 2.0, 3.0, false));
}