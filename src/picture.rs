use crate::colors::ColorCount;
use crate::filters::Filter;
use crate::utils::{BoxShape, Placement, SaveImage};
use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Rgba};

/// Palette struct which holds the image buffer of the palette,
/// number of boxes, box size, and empty space size between boxes
/// This can be empty, or colored, and can be rotated 90 degrees.
pub struct Palette {
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    box_size: BoxShape,
    n_boxes: u32,
    space_size: u32,
}

impl Palette {
    /// Constructor for the Palette which holds `n` number of color boxes
    ///
    /// # Arguments
    /// * side_length - size of each side of the box
    /// * n_boxes - number of color boxes
    /// * space_size - thickness of the empty spaces between boxes
    ///
    /// # Return
    /// This palette
    pub fn new(dims: (u32, u32), n_boxes: u32, space_size: u32) -> Self {
        let width = dims.0 * n_boxes + space_size * (n_boxes - 1);
        let box_size = BoxShape::new(dims.0, dims.1);
        Palette {
            buffer: ImageBuffer::new(width, dims.1),
            box_size,
            n_boxes,
            space_size,
        }
    }

    /// Pains the palette with `n` boxes for top `n` colors.
    /// Algorithm fills the boxes with colored pixels and makes jumps between
    /// colored boxes with empty spaces to keep color boxes separated.
    ///
    /// # Arguments
    /// * n - number of pillars (splits between colors)
    /// * top_colors - vector with top n colors from original img
    pub fn paint_palette(&mut self, top_colors: Vec<&ColorCount>) {
        let mut xp = 0;
        for color in top_colors {
            // fill box with each color
            for _ in 0..self.box_size.width {
                for yp in 0..self.box_size.height {
                    if xp >= self.buffer.width() {
                        break;
                    }
                    self.buffer.put_pixel(xp, yp, color.rgba);
                }
                xp += 1;
            }
            xp += self.space_size; // keep space between boxes
        }
    }

    /// Rotates palette image by 90 degrees.
    pub fn rotate_90degrees(&mut self) {
        let dims = self.buffer.dimensions();
        let mut tmp: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(
                                                    dims.1, dims.0);
        for p in self.buffer.enumerate_pixels() {
            tmp.put_pixel(p.1, p.0, *p.2);
        }
        self.buffer = tmp;
    }

    /// Get the image dimensions of this image buffer in a form of tuple
    pub fn get_dimensions(&self) -> (u32, u32) {
        self.buffer.dimensions()
    }
}

/// Use to build a new image with extended boarders as 'frame'
/// and a palette of `n` most frequently found colors in the original image.
/// Contains `ImageBuffer` and y coordinate that divides picture and palette
pub struct FramedPicture {
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    layer_divider: (u32, u32),
    palette: Palette,
    placement: Placement,
}

impl FramedPicture {
    /// Constructor that takes the dimensions of the original image
    /// and allocates additional space for frame. Then it fills the buffer
    /// with bright beige color and sets the divider y coordinate.
    ///
    /// Also, creates palette which will be adjacent to the original image
    ///
    /// # Arguments
    /// * width - of the image buffer
    /// * height - of the image buffer
    /// * n - # boxes is an optional. Default value is 10
    ///
    /// # Return
    /// * `FramedPicture` struct
    pub fn new(
        width: u32, height: u32, n: Option<u32>, placement: Placement) -> Self {
        let mut box_width = width;
        if placement == Placement::Left || placement == Placement::Right {
            box_width = height;
        }
        let dims = FramedPicture::compute_palette_size(
                            box_width, n.unwrap_or(10));

        let mut _w = 0;
        let mut _h = 0;
        if placement == Placement::Top || placement == Placement::Bottom {
            _w = width + 20;
            _h = height + 30 + dims.0;
        } else {
            _w = width + 30 + dims.0;
            _h = height + 20;
        }

        let tmp: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_fn(
                _w, _h, |_, _| Rgba([255, 252, 234, 1]));
        let p = Palette::new(
            (dims.0, dims.0),
            n.unwrap_or(10),
            dims.1
        );
        let divider = FramedPicture::_set_layers_divider(
                                                    &placement, width, height);

        FramedPicture {
            buffer: tmp,
            layer_divider: divider,
            palette: p,
            placement,
        }
    }

    /// Functions computes and sets the divider point for buffer and palette layers
    ///
    /// # Arguments
    /// * placement - where palette is expected to be placed
    /// * height - of the original image
    /// * width - of the original image
    ///
    /// # Return
    /// Tuple with (x,y) coordinates
    fn _set_layers_divider(
        placement: &Placement, width: u32, height: u32) -> (u32, u32) {
        let mut y_divider = 10;
        let mut x_divider = 10;
        if placement == &Placement::Bottom {
            y_divider += height + 10;
        } else if placement == &Placement::Right {
            x_divider += width + 10;
        }
        (x_divider, y_divider)
    }

    /// Copies supplied dynamic image into this image buffer,
    /// while preserving the allocated space for frame borders
    ///
    /// # Arguments
    /// * size - of the frame
    /// * image - which will be copied into this buffer
    pub fn copy_img_into(&mut self, size: u32, image: &DynamicImage) {
        let mut starting_x = 0;
        let mut starting_y = 0;
        if self.placement == Placement::Left {
            starting_x = self.palette.get_dimensions().0 + size;
        } else if self.placement == Placement::Top {
            starting_y = self.palette.get_dimensions().1 + size;
        }

        for i in image.pixels() {
            let x = i.0 + size + starting_x;
            let y = i.1 + size + starting_y;
            let color = i.2;
            self.buffer.put_pixel(x, y, color);
        }
    }

    /// Calculate the size of each color box for palette layer.
    /// Each box is a square and includes the space for a pillar,
    /// that divides boxes
    ///
    /// # Arguments
    /// * length - of the whole palette
    /// * boxes - number of boxes in the palette
    ///
    /// # Returns
    /// Tuple with the size of square's sides and width of a pillar
    pub fn compute_palette_size(length: u32, boxes: u32) -> (u32, u32) {
        let size = length / (boxes + 1);
        let pillar = (size as f32 * 0.13) as u32;
        (size, pillar)
    }

    pub fn fill_in_palette(&mut self, top_colors: Vec<&ColorCount>) {
        self.palette.paint_palette(top_colors);
        if self.placement == Placement::Left 
        || self.placement == Placement::Right {
            self.palette.rotate_90degrees();
        }
    }

    /// Overlays the palette buffer on top of this buffer.
    /// This is done on an empty space preserved for palette boxes.
    ///
    /// # Arguments
    /// palette - image buffer that contains n boxes separated with pillars.
    pub fn combine_pieces(&mut self) {
        // let divider =
        //     if self.placement == Placement::Top
        //         || self.placement == Placement::Bottom {
        //         self.layer_divider.1
        //     } else { self.layer_divider.0 };
        imageops::overlay(
            &mut self.buffer,
            self.palette.get_buffer(),
            self.layer_divider.0,
            self.layer_divider.1,
        );
    }

    /// Get the image dimensions of this image buffer in a form of tuple
    ///
    /// # Returns
    /// Tuple of x,y dimensions of a given buffer
    pub fn get_dimensions(&self) -> (u32, u32) {
        self.buffer.dimensions()
    }
}

impl Filter for FramedPicture {}

impl SaveImage for Palette {
    fn get_buffer(&self) -> &ImageBuffer<Rgba<u8>, Vec<u8>> {
        &self.buffer
    }
}

impl SaveImage for FramedPicture {
    fn get_buffer(&self) -> &ImageBuffer<Rgba<u8>, Vec<u8>> {
        &self.buffer
    }
}
