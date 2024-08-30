use raytracer::light::Light;
use raytracer::vec3::Vec3;

#[test]
fn test_light_sample() {
    let color = Vec3::new(1.0, 1.0, 1.0, false);
    let position = Vec3::new(0.0, 0.0, 0.0, true);
    let radius = 1.0;
    let light = Light::new(color, position, radius);

    for _ in 0..1000 {
        let sample = light.sample();
        let distance = (sample - position).length();
        assert!(distance <= radius);
    }
}