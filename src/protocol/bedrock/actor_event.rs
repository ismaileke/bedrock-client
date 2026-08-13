use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ActorEvent {
    pub actor_runtime_id: u64,
    pub event_id: u8, //see types/actor_event.rs
    pub event_data: i32,
    pub fire_position: Option<Vec<f32>>,
}

impl Packet for ActorEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDActorEvent.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        stream.put_u8(self.event_id);
        stream.put_var_i32(self.event_data);
        PacketSerializer::write_optional(stream, &self.fire_position, |s, v| PacketSerializer::put_vector3(s, v.clone()));
    }

    fn decode(stream: &mut Reader) -> ActorEvent {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let event_id = stream.get_u8();
        let event_data = stream.get_var_i32();
        let fire_position = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));

        ActorEvent { actor_runtime_id, event_id, event_data, fire_position }
    }
}
