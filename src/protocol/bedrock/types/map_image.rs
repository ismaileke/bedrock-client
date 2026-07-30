use crate::utils::color::Color;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::client_bound_map_item_data::ClientBoundMapItemData;

#[derive(serde::Serialize, Debug)]
pub struct MapImage {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Vec<Color>>,
    pub encoded_pixel_cache: Option<Vec<u8>>
}

impl MapImage {
    pub const MAX_HEIGHT: i32 = 128;
    pub const MAX_WIDTH: i32 = 128;

    pub fn new(pixels: Vec<Vec<Color>>) -> MapImage {
        let mut row_length: Option<i32> = None;
        for row in &pixels {
            if row_length.is_none() {
                row_length = Some(row.len() as i32);
            } else if row.len() as i32 != row_length.unwrap() {
                panic!("All rows must have the same number of pixels");
            }
        }

        if row_length.is_none() {
            panic!("No pixels provided");
        }

        if row_length.unwrap() > Self::MAX_WIDTH {
            panic!("Image width must be at most {} pixels wide", Self::MAX_WIDTH);
        }

        if pixels.len() > Self::MAX_HEIGHT as usize {
            panic!("Image height must be at most {} pixels tall", Self::MAX_HEIGHT);
        }

        let height = pixels.len() as i32;
        let width = row_length.unwrap();

        MapImage {
            width,
            height,
            pixels,
            encoded_pixel_cache: None,
        }
    }

    pub fn read(stream: &mut Stream, height: i32, width: i32) -> MapImage {
        if width > Self::MAX_WIDTH {
            panic!("Image width must be at most {} pixels wide", Self::MAX_WIDTH);
        }
        if height > Self::MAX_HEIGHT {
            panic!("Image height must be at most {} pixels tall", Self::MAX_HEIGHT);
        }

        let mut pixels = Vec::with_capacity(height as usize);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width as usize);
            for _ in 0..width {
                row.push(Color::from_rgba(ClientBoundMapItemData::flip_int_endianness(stream.get_var_u32())));
            }
            pixels.push(row);
        }

        MapImage::new(pixels)
    }

    pub fn write(&mut self, stream: &mut Stream) {
        if self.encoded_pixel_cache.is_none() {
            let mut buffer = Stream::new(Vec::new(), 0);
            for y in 0..self.height {
                for x in 0..self.width {
                    buffer.put_var_u32(ClientBoundMapItemData::flip_int_endianness(self.pixels[y as usize][x as usize].to_rgba()));
                }
            }
            self.encoded_pixel_cache = Some(buffer.get_buffer().to_vec());
        }

        if let Some(data) = self.encoded_pixel_cache.as_ref() {
            stream.put(data.to_vec());
        }
    }
}
