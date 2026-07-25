use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ActorEvent {
    pub actor_runtime_id: u64,
    pub event_id: u8, //see types/actor_event.rs
    pub event_data: i32,
    pub fire_position: Option<Vec<f32>>,
}

impl Packet for ActorEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDActorEvent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_byte(self.event_id);
        stream.put_var_i32(self.event_data);
        PacketSerializer::write_optional(&mut stream, &self.fire_position, |s, v| PacketSerializer::put_vector3(s, v.clone()));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ActorEvent {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let event_id = stream.get_byte();
        let event_data = stream.get_var_i32();
        let fire_position = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));

        ActorEvent { actor_runtime_id, event_id, event_data, fire_position }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
