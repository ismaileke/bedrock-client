use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ChangeDimension {
    pub dimension: i32,
    pub position: Vec<f32>,
    pub respawn: bool,
    pub loading_screen_id: Option<u32>,
}

impl Packet for ChangeDimension {
    fn id(&self) -> u16 {
        BedrockPacketType::IDChangeDimension.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.dimension);
        PacketSerializer::put_vector3(stream, self.position.clone());
        stream.put_bool(self.respawn);
        PacketSerializer::write_optional(stream, &self.loading_screen_id, |s, v| s.put_u32_le(*v));
    }

    fn decode(stream: &mut Reader) -> ChangeDimension {
        let dimension = stream.get_var_i32();
        let position = PacketSerializer::get_vector3(stream);
        let respawn = stream.get_bool();
        let loading_screen_id = PacketSerializer::read_optional(stream, |s| s.get_u32_le());

        ChangeDimension { dimension, position, respawn, loading_screen_id }
    }
}
