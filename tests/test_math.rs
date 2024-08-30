use raytracer::math::*;

#[test]
fn test_div_up() {
    assert_eq!(div_up(10, 3), 4);
    assert_eq!(div_up(9, 3), 3);
    assert_eq!(div_up(0, 1), 0);
    assert_eq!(div_up(1, 1), 1);
    assert_eq!(div_up(1, 2), 1);
}

#[test]
fn test_clamp() {
    assert_eq!(clamp(5.0, 1.0, 10.0), 5.0);
    assert_eq!(clamp(0.0, 1.0, 10.0), 1.0);
    assert_eq!(clamp(15.0, 1.0, 10.0), 10.0);
    assert_eq!(clamp(-5.0, -10.0, 0.0), -5.0);
    assert_eq!(clamp(-15.0, -10.0, 0.0), -10.0);
}

#[test]
fn test_transpose() {
    let matrix = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ];
    let expected = [
        [1.0, 5.0, 9.0, 13.0],
        [2.0, 6.0, 10.0, 14.0],
        [3.0, 7.0, 11.0, 15.0],
        [4.0, 8.0, 12.0, 16.0],
    ];
    assert_eq!(transpose(&matrix), expected);
    assert_eq!(transpose(&transpose(&matrix)), matrix);
}

#[test]
fn test_multiply() {
    let a = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ];
    let b = [
        [17.0, 18.0, 19.0, 20.0],
        [21.0, 22.0, 23.0, 24.0],
        [25.0, 26.0, 27.0, 28.0],
        [29.0, 30.0, 31.0, 32.0],
    ];
    let expected = [
        [250.0, 260.0, 270.0, 280.0],
        [618.0, 644.0, 670.0, 696.0],
        [986.0, 1028.0, 1070.0, 1112.0],
        [1354.0, 1412.0, 1470.0, 1528.0],
    ];
    assert_eq!(multiply(&a, &b), expected);
}