extern crate paleatra;

use paleatra::picture;

#[test]
fn test_framedpic_constructor() {
    let _pic = picture::FramedPicture::new(200, 100, Some(10));
    assert_eq!(1, 1);
}

#[test]
fn test_compute_palette_size() {
    let pal = picture::FramedPicture::compute_palette_size(100, 9);
    assert_eq!(pal.0, 10); // test size
    assert_eq!(pal.1, 1); // test pillar
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
    let pic = picture::FramedPicture::new(200, 100, Some(10));
    let buffer = pic.create_palette();
    assert_eq!(buffer.width(), 200);
    assert_eq!(buffer.height(), 28);
}