use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct LevelEvent {
    pub event_id: i32, //see types/level_event
    pub position: Vec<f32>,
    pub event_data: i32,
}

impl Packet for LevelEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLevelEvent.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.event_id);
        PacketSerializer::put_vector3(stream, &self.position);
        stream.put_var_i32(self.event_data);
    }

    fn decode(stream: &mut Reader) -> LevelEvent {
        let event_id = stream.get_var_i32();
        let position = PacketSerializer::get_vector3(stream);
        let event_data = stream.get_var_i32();

        LevelEvent {
            event_id,
            position,
            event_data,
        }
    }
}
