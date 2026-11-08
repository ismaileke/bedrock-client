use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PassengerOfBlockData {
    pub block_position: Vec<i32>,
    pub offset: Vec<f32>,
    pub rotation: f32,
    pub rotation_limit: f32,
    pub emote_type: u8
}

impl PassengerOfBlockData {
    pub const EMOTE_TYPE_STANDING: u8 = 0;
    pub const EMOTE_TYPE_RIDING: u8 = 1;
    pub const EMOTE_TYPE_LAYING: u8 = 2;

    pub fn read(stream: &mut Reader) -> PassengerOfBlockData {
        let block_position = PacketSerializer::get_block_pos(stream);
        let offset = PacketSerializer::get_vector3(stream);
        let rotation = stream.get_f32_le();
        let rotation_limit = stream.get_f32_le();
        let emote_type = stream.get_u8();
        PassengerOfBlockData { block_position, offset, rotation, rotation_limit , emote_type }

    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, &self.block_position);
        PacketSerializer::put_vector3(stream, &self.offset);
        stream.put_f32_le(self.rotation);
        stream.put_f32_le(self.rotation_limit);
        stream.put_u8(self.emote_type);
    }
}