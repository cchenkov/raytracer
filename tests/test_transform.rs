use raytracer::vec3::Vec3;
use raytracer::ray::Ray;
use raytracer::transform::{Transform, translation_matrix};

#[test]
fn translation_does_not_affect_vec3() {
    let vec1 = Vec3::new(1.0, 1.0, 1.0, false);
    let translation1 = translation_matrix(&vec1);
    let vec = Vec3::new(5.0, 5.0, 5.0, false);

    let transformed = vec.transform(&translation1.inv);

    assert_eq!(vec.x(), transformed.x());
    assert_eq!(vec.y(), transformed.y());
    assert_eq!(vec.z(), transformed.z());
}

#[test]
fn translate_ray() {
    let vec1 = Vec3::new(1.0, 1.0, 1.0, false);
    let translation1 = translation_matrix(&vec1);

    let ray = Ray::new(Vec3::new(1.0, 1.0, 1.0, true), Vec3::new(5.0, 5.0, 5.0, false));
    let transformed = ray.transform(&translation1.mat);

    // assert_eq!(transformed.origin().x(), 2.0);
    // assert_eq!(transformed.origin().y(), 2.0);
    // assert_eq!(transformed.origin().z(), 2.0);
    assert_eq!(ray.direction().x(), transformed.direction().x());
    assert_eq!(ray.direction().y(), transformed.direction().z());
    assert_eq!(ray.direction().y(), transformed.direction().z());
}