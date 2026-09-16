use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug, Clone)]
pub struct SubChunkHeightMapInfo {
    pub heights: Vec<i8>,
}

impl SubChunkHeightMapInfo {
    const ENTRY_COUNT: u32 = 272;
    const RUN_LENGTH: u32 = 16;

    pub fn new(heights: [i8; Self::ENTRY_COUNT as usize]) -> Self {
        Self {
            heights: heights.to_vec(),
        }
    }

    pub fn get_heights(&self) -> &[i8] {
        &self.heights
    }

    pub fn get_height(&self, x: usize, z: usize) -> i8 {
        let index = ((z & 0xf) << 4) | (x & 0xf);
        self.heights[index]
    }

    pub fn all_too_low() -> Self {
        Self {
            heights: vec![-1i8; Self::ENTRY_COUNT as usize],
        }
    }

    pub fn all_too_high() -> Self {
        Self {
            heights: vec![16i8; Self::ENTRY_COUNT as usize],
        }
    }

    pub fn is_all_too_low(&self) -> bool {
        self.heights.iter().all(|&h| h < 0)
    }

    pub fn is_all_too_high(&self) -> bool {
        self.heights.iter().all(|&h| h > 15)
    }

    pub fn read(stream: &mut Reader) -> SubChunkHeightMapInfo {
        let mut heights = Vec::new();
        let mut i: u32 = 0;

        while i < Self::ENTRY_COUNT {
            let run_length = stream.get_var_u32();
            if run_length != Self::RUN_LENGTH { panic!("Expected heightmap run length of {}, got {}", Self::RUN_LENGTH, run_length) }
            for _ in 0..Self::RUN_LENGTH {
                heights.push(stream.get_i8());
            }

            i += Self::RUN_LENGTH;
        }

        for _ in 0..256 {
            heights.push(stream.get_u8() as i8);
        }
        SubChunkHeightMapInfo { heights }
    }

    pub fn write(&self, stream: &mut Writer) {
        let mut i: u32 = 0;

        while i < Self::ENTRY_COUNT {
            stream.put_var_u32(Self::RUN_LENGTH);
            for j in 0..Self::RUN_LENGTH {
                stream.put_i8(self.heights[(i + j) as usize]);
            }

            i += Self::RUN_LENGTH;
        }
    }
}
