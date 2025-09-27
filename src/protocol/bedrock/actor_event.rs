use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ActorEvent {
    pub actor_runtime_id: u64,
    pub event_id: u8, //see types/actor_event.rs
    pub event_data: i32
}

pub fn new(actor_runtime_id: u64, event_id: u8, event_data: i32) -> ActorEvent {
    ActorEvent { actor_runtime_id, event_id, event_data }
}

impl Packet for ActorEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDActorEvent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_byte(self.event_id);
        stream.put_var_int(self.event_data);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ActorEvent {
        let mut stream = Stream::new(bytes, 0);

        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let event_id = stream.get_byte();
        let event_data = stream.get_var_int();

        ActorEvent { actor_runtime_id, event_id, event_data }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Event ID: {}", self.event_id);
        println!("Event Data: {}", self.event_data);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
