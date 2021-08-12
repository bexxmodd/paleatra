use image::{ImageBuffer, Rgb};

pub trait Filter {
    fn grayscale(img: &ImageBuffer) -> ImageBuffer<Rgb, Vec<u8>> {
        let grayscale = img.clone();
        for p in img.enumerate_pixels() {
            let r = *p.2.0;
            let g = *p.2.1;
            let b = *p.2.2;

            let gray = (r + g + b) / 3;
            // grayscale.put_pixel(p.0, p.1, Rgb<)
        }
    }
}