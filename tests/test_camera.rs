use raytracer::camera::Camera;
use raytracer::vec3::Vec3;
use raytracer::ray::Ray;

#[test]
fn test_camera_get_ray() {
    let look_from = Vec3::new(0.0, 0.0, 0.0, true);
    let look_at = Vec3::new(0.0, 0.0, -1.0, true);
    let vup = Vec3::new(0.0, 1.0, 0.0, false);
    let vfov = 90.0;
    let aspect_ratio = 16.0 / 9.0;

    let camera = Camera::new(look_from, look_at, vup, vfov, aspect_ratio);

    let ray = camera.get_ray(0.5, 0.5);
    let expected_direction = Vec3::new(0.0, 0.0, -1.0, true);
    assert_eq!(ray.direction().normalized(), expected_direction);
}