use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct BiomeNoiseBlockSpecifier {
    pub noise: String,
    pub threshold: f32,
    pub min: f32,
    pub max: f32,
    pub block: u32
}

impl BiomeNoiseBlockSpecifier {
    pub fn new(
        noise: String,
        threshold: f32,
        min: f32,
        max: f32,
        block: u32
    ) -> Self {
        BiomeNoiseBlockSpecifier { noise, threshold, min, max, block }
    }

    pub fn read(stream: &mut Reader) -> BiomeNoiseBlockSpecifier {
        let noise = PacketSerializer::get_string(stream);
        let threshold = stream.get_f32_le();
        let min = stream.get_f32_le();
        let max = stream.get_f32_le();
        let block = stream.get_u32_le();

        BiomeNoiseBlockSpecifier::new(noise, threshold, min, max, block)
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.noise.clone());
        stream.put_f32_le(self.threshold);
        stream.put_f32_le(self.min);
        stream.put_f32_le(self.max);
        stream.put_u32_le(self.block);
    }
}
