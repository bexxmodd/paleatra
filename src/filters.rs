use image::DynamicImage;

pub trait Filter {
    fn grayscale(img: &DynamicImage) -> DynamicImage {
        img.grayscale()
    }

    fn invert_colors(img: &DynamicImage) -> DynamicImage {
        let mut inv_img = img.clone();
        inv_img.invert();
        inv_img
    }
}
