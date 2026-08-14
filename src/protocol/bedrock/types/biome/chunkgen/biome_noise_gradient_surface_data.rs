use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::biome::chunkgen::biome_noise_block_specifier::BiomeNoiseBlockSpecifier;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct BiomeNoiseGradientSurfaceData {
    pub non_replaceable_blocks: Vec<u32>,
    pub gradient_blocks: Vec<BiomeNoiseBlockSpecifier>,
    pub noise_seed: String,
    pub first_octave: bool,
    pub amplitudes: Vec<f32>

}
impl BiomeNoiseGradientSurfaceData {
    pub fn new(
        non_replaceable_blocks: Vec<u32>,
        gradient_blocks: Vec<BiomeNoiseBlockSpecifier>,
        noise_seed: String,
        first_octave: bool,
        amplitudes: Vec<f32>
    ) -> Self {
        BiomeNoiseGradientSurfaceData { non_replaceable_blocks, gradient_blocks, noise_seed, first_octave, amplitudes }
    }

    pub fn read(stream: &mut Reader) -> BiomeNoiseGradientSurfaceData {
        let mut count = stream.get_var_u32();
        let mut non_replaceable_blocks = Vec::new();
        for _ in 0..count {
            non_replaceable_blocks.push(stream.get_u32_le());
        }
        count = stream.get_var_u32();
        let mut gradient_blocks = Vec::new();
        for _ in 0..count {
            gradient_blocks.push(BiomeNoiseBlockSpecifier::read(stream));
        }
        let noise_seed = PacketSerializer::get_string(stream);
        let first_octave = stream.get_bool();
        count = stream.get_var_u32();
        let mut amplitudes = Vec::new();
        for _ in 0..count {
            amplitudes.push(stream.get_f32_le());
        }

        BiomeNoiseGradientSurfaceData::new(non_replaceable_blocks, gradient_blocks, noise_seed, first_octave, amplitudes)
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.non_replaceable_blocks.len() as u32);
        for non_replaceable_block in &self.non_replaceable_blocks {
            stream.put_u32_le(*non_replaceable_block);
        }
        stream.put_var_u32(self.gradient_blocks.len() as u32);
        for gradient_block in &self.gradient_blocks {
            gradient_block.write(stream);
        }
        PacketSerializer::put_string(stream, &self.noise_seed);
        stream.put_bool(self.first_octave);
        stream.put_var_u32(self.amplitudes.len() as u32);
        for amplitude in &self.amplitudes {
            stream.put_f32_le(*amplitude);
        }
    }
}
