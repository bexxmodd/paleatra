extern crate paleatra;

use paleatra::picture;

#[test]
fn test_framedpic_constructor() {
    let _pic = picture::FramedPicture::new(200, 100, Some(10));
    assert!(true);
}

#[test]
fn test_compute_palette_size() {
    let pic = picture::FramedPicture::compute_palette_size(100, 9);
    assert_eq!(pic.0, 10); // test size
    assert_eq!(pic.1, 1); // test pillar
}

#[test]
fn test_get_dimensions() {
    let pic = picture::FramedPicture::new(200, 100, Some(10));
    let dims = pic.get_dimensions();
    assert_eq!(dims.0, 220);
    assert_eq!(dims.1, 148);
}

#[test]
fn test_create_palette() {
    let pal = picture::Palette::new(50, 10, 5);
    assert_eq!(pal.get_dimensions().0, 545);
    assert_eq!(pal.get_dimensions().1, 50);
}

#[test]
fn test_palette_rotation() {
    let mut pal = picture::Palette::new(50, 10, 5);
    pal.rotate_90degrees();
    assert_eq!(pal.get_dimensions().1, 545);
    assert_eq!(pal.get_dimensions().0, 50);
}