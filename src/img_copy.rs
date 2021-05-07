use image::{ImageBuffer, DynamicImage, Rgba, GenericImageView, imageops};
use crate::colors::ColorCount;

pub struct FramedPicture {
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    divider: u32,
}

impl FramedPicture {
    pub fn new(width: u32, height: u32, div: u32) -> Self {
        let tmp: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(
            width, height, |_,_| { Rgba([255, 252, 234, 1]) });

        FramedPicture {
            buffer: tmp,
            divider: div,
        }
    }

    pub fn draw_palette(&mut self, n: u32,
                        top_colors: &Vec<(u32, &ColorCount)>)
                        -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let dims =
            FramedPicture::compute_palette_size(
                self.buffer.width() - 20, 10);
        let pwidth = dims.0 * n + dims.1 * 9;
        let pheight = dims.0 + 10;

        let mut palette = ImageBuffer::new(pwidth, pheight);

        let mut xp = 0;
        for color in top_colors {
            for _ in 0..dims.0 {
                let mut yp = 0;
                while yp < dims.0 {
                    palette.put_pixel(xp, yp, color.1.rgba);
                    yp += 1;
                }
                xp += 1;
            }
            xp += dims.1; // skip the frame's vertical line
        }
        palette
    }

    pub fn copy_img_into(&mut self, size: u32, image: &DynamicImage) {
        for i in image.pixels() {
            let x = i.0 + size;
            let y = i.1 + size;
            let color = i.2;
            self.buffer.put_pixel(x, y, color);
        }
    }

    pub fn save_img(&self, path: &str) {
        self.buffer.save(path).unwrap();
    }

    pub fn compute_palette_size(length: u32, boxes: u32) -> (u32, u32){
        let size = (length - 20) / (boxes + 1);
        let pillar = (size as f32 * 0.13) as u32;
        (size, pillar)
    }

    pub fn stick_piece(&mut self, palette: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
        imageops::overlay(
            &mut self.buffer, palette, 10, self.divider);
    }
}