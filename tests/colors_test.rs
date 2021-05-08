extern crate paleatra;

use paleatra::colors;
use image::Rgba;

#[test]
fn test_color_construct() {
    let w = Rgba([255, 255, 255, 1]);
    let white = colors::ColorCount::new(w);
    assert_eq!(white.count, 1);
}

#[test]
fn test_color_count_increment() {
    let w = Rgba([255, 255, 255, 1]);
    let mut white = colors::ColorCount::new(w);
    for _ in 0..10 {
        white.increment_count();
    }
    assert_eq!(white.count, 11);
}

#[test]
fn test_color_generate_hex() {
    let r = Rgba([255, 0, 0, 1]);
    let red = colors::ColorCount::new(r);
    assert_eq!("xFF0000", red.hex);
}

#[test]
fn test_color_measure_diff() {
    let r1 = Rgba([255, 255, 255, 1]);
    let r3 = Rgba([250, 250, 250, 1]);
    let red1 = colors::ColorCount::new(r1);
    let red2 = colors::ColorCount::new(r1);
    let red3 = colors::ColorCount::new(r3);

    let diff0 = red1.measure_diff(&red2);
    let diff1 = red1.measure_diff(&red3);

    assert_eq!(diff0, 0);
    assert_eq!(diff1, 25);
}