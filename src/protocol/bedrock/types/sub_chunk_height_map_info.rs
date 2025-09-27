use binary_utils::binary::Stream;

#[derive(Debug)]
#[derive(Clone)]
pub struct SubChunkHeightMapInfo {
    heights: [i8; 256]
}

impl SubChunkHeightMapInfo {
    pub fn new(heights: [i8; 256]) -> Self {
        Self { heights }
    }

    pub fn get_heights(&self) -> &[i8; 256] {
        &self.heights
    }

    pub fn get_height(&self, x: usize, z: usize) -> i8 {
        let index = ((z & 0xf) << 4) | (x & 0xf);
        self.heights[index]
    }

    pub fn all_too_low() -> Self {
        Self::new([-1i8; 256])
    }

    pub fn all_too_high() -> Self {
        Self::new([16i8; 256])
    }

    pub fn is_all_too_low(&self) -> bool {
        self.heights.iter().all(|&h| h < 0)
    }

    pub fn is_all_too_high(&self) -> bool {
        self.heights.iter().all(|&h| h > 15)
    }

    pub fn read(stream: &mut Stream) -> SubChunkHeightMapInfo {
        let mut heights = [0i8; 256];
        for i in 0..256 {
            heights[i] = stream.get_byte() as i8;
        }
        SubChunkHeightMapInfo::new(heights)
    }

    pub fn write(&self, stream: &mut Stream) {
        for &height in &self.heights {
            stream.put_byte(height as u8);
        }
    }
}