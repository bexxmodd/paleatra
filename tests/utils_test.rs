extern crate paleatra;

use paleatra::utils;
use std::collections::HashSet;
use paleatra::colors::ColorCount;
use image::Rgba;

#[test]
fn test_get_most_freq() {
    let mut set: HashSet<ColorCount> = HashSet::new();
    let mut color;
    for i in (0..200).step_by(10) {
        color = ColorCount::new(Rgba([i, 10 + i, 20 + i, 0]));
        for _ in 0..5 + i {
            color.increment_count();
        }
        set.insert(color);
    }

    let freq = utils::get_most_freq(&set, 10);
    let prev = &freq[0];
    for c in &freq[1..] {
        assert!(c.0 <= prev.0, "Color counts are not sorted in descending order");
    }
}

#[test]
fn test_get_colors_from() {
    let img = image::open("img/test_color.jpg").unwrap();
    let set = utils::get_colors_from(&img);
    assert_eq!(set.len(), 1);

    let c = ColorCount::new(Rgba([255,20,0,0]));
    let found = set.get(&c);
    assert_eq!(found.unwrap().count, 10000);
}