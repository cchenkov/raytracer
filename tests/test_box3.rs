use raytracer::box3::Box3;
use raytracer::ray::Ray;
use raytracer::vec3::Vec3;
use raytracer::hit::Hit;
use raytracer::material::Material;

use Vec3 as Color;

#[test]
fn test_normal_at() {
    let min_bound = Vec3::new(-1.0, -1.0, -1.0, true);
    let max_bound = Vec3::new(1.0, 1.0, 1.0, true);
    let material = Material::new(Color::new(0.0, 0.0, 0.0, false), 0.0, 0.0);
    let transform_matrix = None;

    let box3 = Box3::new(min_bound, max_bound, material, transform_matrix);

    let normal = box3.normal_at(Vec3::new(0.0, 0.5, 1.0, true));
    assert_eq!(normal, Vec3::new(0.0, 0.0, 1.0, false).normalized());

    let normal = box3.normal_at(Vec3::new(0.0, 0.5, -1.0, true));
    assert_eq!(normal, Vec3::new(0.0, 0.0, 1.0, false).normalized());

    let normal = box3.normal_at(Vec3::new(0.0, 1.0, 0.5, true));
    assert_eq!(normal, Vec3::new(0.0, 1.0, 0.0, false).normalized());

    let normal = box3.normal_at(Vec3::new(0.0, -1.0, 0.5, true));
    assert_eq!(normal, Vec3::new(0.0, 1.0, 0.0, false).normalized());

    let normal = box3.normal_at(Vec3::new(1.0, 0.0, 0.5, true));
    assert_eq!(normal, Vec3::new(1.0, 0.0, 0.0, false).normalized());

    let normal = box3.normal_at(Vec3::new(-1.0, 0.0, 0.5, true));
    assert_eq!(normal, Vec3::new(1.0, 0.0, 0.0, false).normalized());
}

#[test]
fn test_hit_inside() {
    let min_bound = Vec3::new(-1.0, -1.0, -1.0, true);
    let max_bound = Vec3::new(1.0, 1.0, 1.0, true);
    let material = Material::new(Color::new(0.0, 0.0, 0.0, false), 0.0, 0.0);
    let transform_matrix = None;

    let box3 = Box3::new(min_bound, max_bound, material, transform_matrix);

    let ray_inside = Ray::new(Vec3::new(0.0, 0.0, 0.0, true), Vec3::new(0.0, 0.0, 1.0, false));
    let hit_record = box3.hit(&ray_inside, 0.0, f64::INFINITY);
    assert!(hit_record.is_some());
}

#[test]
fn test_hit_outside() {
    let min_bound = Vec3::new(-1.0, -1.0, -1.0, true);
    let max_bound = Vec3::new(1.0, 1.0, 1.0, true);
    let material = Material::new(Color::new(0.0, 0.0, 0.0, false), 0.0, 0.0);
    let transform_matrix = None;

    let box3 = Box3::new(min_bound, max_bound, material, transform_matrix);

    let ray_outside = Ray::new(Vec3::new(0.0, 0.0, 2.0, true), Vec3::new(0.0, 0.0, -1.0, false));
    let hit_record = box3.hit(&ray_outside, 0.0, f64::INFINITY);
    assert!(hit_record.is_some());
}

#[test]
fn test_hit_miss() {
    let min_bound = Vec3::new(-1.0, -1.0, -1.0, true);
    let max_bound = Vec3::new(1.0, 1.0, 1.0, true);
    let material = Material::new(Color::new(0.0, 0.0, 0.0, false), 0.0, 0.0);
    let transform_matrix = None;

    let box3 = Box3::new(min_bound, max_bound, material, transform_matrix);

    let ray_miss = Ray::new(Vec3::new(2.0, 2.0, 2.0, true), Vec3::new(0.0, 0.0, -1.0, false));
    let hit_record = box3.hit(&ray_miss, 0.0, f64::INFINITY);
    assert!(hit_record.is_none());
}
