use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct MovementEffect {
    pub actor_runtime_id: u64,
    pub effect_type: u32, //see types/movement_effect_type.rs
    pub duration: u32,
    pub tick: u64
}

pub fn new(actor_runtime_id: u64, effect_type: u32, duration: u32, tick: u64) -> MovementEffect {
    MovementEffect { actor_runtime_id, effect_type, duration, tick }
}

impl Packet for MovementEffect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMovementEffect.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_unsigned_var_int(self.effect_type);
        stream.put_unsigned_var_int(self.duration);
        stream.put_unsigned_var_long(self.tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> MovementEffect {
        let mut stream = Stream::new(bytes, 0);

        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let effect_type = stream.get_unsigned_var_int();
        let duration = stream.get_unsigned_var_int();
        let tick = stream.get_unsigned_var_long();

        MovementEffect { actor_runtime_id, effect_type, duration, tick }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Effect Type: {}", self.effect_type);
        println!("Duration: {}", self.duration);
        println!("Tick: {}", self.tick);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
