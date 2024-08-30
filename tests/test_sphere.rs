
use raytracer::sphere::Sphere;
use raytracer::vec3::Vec3;
use raytracer::ray::Ray;
use raytracer::hit::Hit;
use raytracer::material::Material;

use Vec3 as Color;

#[test]
fn test_ray_misses_sphere() {
    let center = Vec3::new(0.0, -1.0, 0.0, true);
    let radius = 0.5;
    let material = Material::new(Color::new(0.0, 0.0, 0.0, false), 0.0, 0.0);
    let sphere = Sphere::new(center, radius, material, None);

    let origin = Vec3::new(0.0, 0.0, 1.0, true);
    let direction = Vec3::new(0.0, 0.0, -1.0, false).normalized();
    let ray = Ray::new(origin, direction);

    let hit_record = sphere.hit(&ray, 0.0, f64::INFINITY);
    assert!(hit_record.is_none());
}

#[test]
fn test_ray_hits_sphere_at_two_points() {
    let center = Vec3::new(0.0, 0.0, 0.0, true);
    let radius = 0.5;
    let material = Material::new(Color::new(0.0, 0.0, 0.0, false), 0.0, 0.0);
    let sphere = Sphere::new(center, radius, material, None);

    let origin = Vec3::new(0.0, 0.0, 1.0, true);
    let direction = Vec3::new(0.0, 0.0, -1.0, false);
    let ray = Ray::new(origin, direction);

    let hit_record = sphere.hit(&ray, 0.0, f64::INFINITY);
    assert!(hit_record.is_some());

    let hit_record = hit_record.unwrap();
    assert!(hit_record.t_min > 0.0);
    assert_eq!(hit_record.point, Vec3::new(0.0, 0.0, 0.5, true));
    assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, 1.0, true));
}

#[test]
fn test_ray_hits_sphere_at_one_point() {
    let center = Vec3::new(0.0, -0.5, 0.0, true);
    let radius = 0.5;
    let material = Material::new(Color::new(0.0, 0.0, 0.0, false), 0.0, 0.0);
    let sphere = Sphere::new(center, radius, material, None);

    let origin = Vec3::new(0.0, 0.0, 1.0, true);
    let direction = Vec3::new(0.0, 0.0, -1.0, false);
    let ray = Ray::new(origin, direction);

    let hit_record = sphere.hit(&ray, 0.0, f64::INFINITY);
    assert!(hit_record.is_some());

    let hit_record = hit_record.unwrap();
    assert!(hit_record.t_min > 0.0);
    assert_eq!(hit_record.point, Vec3::new(0.0, 0.0, 0.0, true));
    assert_eq!(hit_record.normal, Vec3::new(0.0, -1.0, 0.0, true));
}

#[test]
fn test_ray_origin_inside_sphere() {
    let center = Vec3::new(0.0, 0.0, 0.0, true);
    let radius = 0.5;
    let material = Material::new(Color::new(0.0, 0.0, 0.0, false), 0.0, 0.0);
    let sphere = Sphere::new(center, radius, material, None);

    let origin = Vec3::new(0.0, 0.0, 0.0, true);
    let direction = Vec3::new(0.0, 0.0, -1.0, false);
    let ray = Ray::new(origin, direction);

    let hit_record = sphere.hit(&ray, 0.0, f64::INFINITY);
    assert!(hit_record.is_some());

    let hit_record = hit_record.unwrap();
    assert!(hit_record.t_min > 0.0);
    assert_eq!(hit_record.point, Vec3::new(0.0, 0.0, -0.5, true));
    assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, 1.0, true));
}