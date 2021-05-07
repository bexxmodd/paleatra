use image::{ImageBuffer, DynamicImage, Rgba, GenericImageView, imageops};
use crate::colors::ColorCount;

/// Usage to build an new image with extended boarders as frame
/// and palette of n most frequently found colors in the original image.
/// Contains ImageBuffer and y coordinate to divide picture and palette
pub struct FramedPicture {
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    y_divider: u32,
}

impl FramedPicture {

    /// Constructor that takes the dimensions of the original image
    /// and allocates additional space for frame. Then it fills the buffer
    /// with bright beige color and sets the divider y coordinate.
    ///
    /// # Arguments
    /// * width - of the image buffer
    /// * height - of the image buffer
    ///
    /// # Return
    /// * FramedPicture struct
    pub fn new(width: u32, height: u32, n: Option<u32>) -> Self {
        let dims = FramedPicture::compute_palette_size(
            width, n.unwrap_or(10));
        let w = width + 20;
        let h = height + 30 + dims.0;
        let tmp: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(
            w, h, |_,_| { Rgba([255, 252, 234, 1]) });

        FramedPicture {
            buffer: tmp,
            y_divider: height + 20,
        }
    }

    /// Draws the palette with n boxes for top n colors.
    /// Algorithm draws color pixels one by one from given vector
    /// and makes jumps between colors with width of a pillar.
    ///
    /// # Arguments
    /// * n - number of pillars (splits between colors)
    /// * top_colors - vector with top n colors from original img
    ///
    /// # Returns
    /// Image which is a palette with n boxes and empty spaces in between
    pub fn draw_palette(&mut self, n: u32,
                        top_colors: &Vec<(u32, &ColorCount)>)
                        -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let dims = FramedPicture::compute_palette_size(
            self.buffer.width() - 20, n);

        let pwidth = self.buffer.width() - 20;
        let pheight = dims.0 + 10;

        let mut palette = ImageBuffer::new(pwidth, pheight);

        let mut xp = 0;
        for color in top_colors { // fill box with each color
            for _ in 0..dims.0 {
                let mut yp = 0;
                while yp < dims.0 {
                    if xp >= pwidth { break; }
                    palette.put_pixel(xp, yp, color.1.rgba);
                    yp += 1;
                }
                xp += 1;
            }
            xp += dims.1; // keep space between boxes
        }
        palette
    }

    /// Copies supplied dynamic image into this image buffer,
    /// while preserving the allocated space for frame borders
    ///
    /// # Arguments
    /// * size - of the frame
    /// * image - which will be copied into this buffer
    pub fn copy_img_into(&mut self, size: u32, image: &DynamicImage) {
        for i in image.pixels() {
            let x = i.0 + size;
            let y = i.1 + size;
            let color = i.2;
            self.buffer.put_pixel(x, y, color);
        }
    }

    /// Save this buffer as an image
    /// image format will be set based on provided path,
    /// which is expected to include the name of the new file
    ///
    /// # Argument
    /// * path - full or relative path with new file name and format
    pub fn save_img(&self, path: &str) {
        self.buffer.save(path).unwrap();
    }

    /// Calculate the size of each color box for palette layer.
    /// Each box is a square and includes the space for a pillar, that divides boxes
    ///
    /// # Arguments
    /// * length - of the whole palette
    /// * boxes - number of boxes in the palette
    ///
    /// # Returns
    /// Tuple with the size of square's sides and width of a pillar
    pub fn compute_palette_size(length: u32, boxes: u32) -> (u32, u32){
        let size = length / (boxes + 1);
        let pillar = (size as f32 * 0.13) as u32;
        (size, pillar)
    }

    /// Overlays the palette buffer on top of this buffer.
    /// This is done on an empty space preserved in this buffer for palette boxes.
    ///
    /// # Arguments
    /// palette - image buffer that contains n boxes separated with pillars.
    pub fn stick_piece(&mut self, palette: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
        imageops::overlay(
            &mut self.buffer, palette, 10, self.y_divider);
    }
}