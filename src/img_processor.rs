use image::{ImageBuffer, DynamicImage, Rgba, GenericImageView};
use crate::colors::ColorCount;

pub fn copy_img_into(frame: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
                     size: u32, image: &DynamicImage,) {
    for i in image.pixels() {
        let x = i.0 + size;
        let y = i.1 + size;
        let color = i.2;
        frame.put_pixel(x, y, color);
    }
}

pub fn draw_palette(dims: (u32, u32), n: u32,
                    top_colors: &Vec<(u32, &ColorCount)>)
                    -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let pwidth = dims.0 * n + dims.1 * 9;
    let pheight = dims.0 + 10;
    println!("Width {} Pilar {}", dims.0, dims.1);

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

pub fn compute_palette_size(img_dims: &(u32, u32), boxes: u32) -> (u32, u32) {
    let size: u32;
    if img_dims.0 > img_dims.1 {
        size = img_dims.0 / (boxes + 1);
    } else {
        size = img_dims.1 / (boxes + 1);
    }
    let pillar = (size as f32 * 0.13) as u32;
    (size, pillar)
}