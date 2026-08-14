use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerPlayerPostMovePosition {
    pub position: Vec<f32>,
}

impl Packet for ServerPlayerPostMovePosition {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerPlayerPostMovePosition.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_vector3(stream, &self.position);
    }

    fn decode(stream: &mut Reader) -> ServerPlayerPostMovePosition {
        let position = PacketSerializer::get_vector3(stream);

        ServerPlayerPostMovePosition { position }
    }
}
