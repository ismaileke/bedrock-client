use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct MobEffect {
    pub actor_runtime_id: u64,
    pub event_id: u8,
    pub effect_id: i32,
    pub amplifier: i32,
    pub particles: bool,
    pub duration: i32,
    pub tick: u64
}

pub fn new(actor_runtime_id: u64, event_id: u8, effect_id: i32, amplifier: i32, particles: bool, duration: i32, tick: u64) -> MobEffect {
    MobEffect { actor_runtime_id, event_id, effect_id, amplifier, particles, duration, tick }
}

impl MobEffect {
    pub const EVENT_ADD: u8 = 1;
    pub const EVENT_MODIFY: u8 = 2;
    pub const EVENT_REMOVE: u8 = 3;
}

impl Packet for MobEffect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMobEffect.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_byte(self.event_id);
        stream.put_var_int(self.effect_id);
        stream.put_var_int(self.amplifier);
        stream.put_bool(self.particles);
        stream.put_var_int(self.duration);
        stream.put_unsigned_var_long(self.tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> MobEffect {
        let mut stream = Stream::new(bytes, 0);

        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let event_id = stream.get_byte();
        let effect_id = stream.get_var_int();
        let amplifier = stream.get_var_int();
        let particles = stream.get_bool();
        let duration = stream.get_var_int();
        let tick = stream.get_unsigned_var_long();

        MobEffect { actor_runtime_id, event_id, effect_id, amplifier, particles, duration, tick }
    }
    
    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Event ID: {}", self.event_id);
        println!("Effect ID: {}", self.effect_id);
        println!("Amplifier: {}", self.amplifier);
        println!("Particles: {}", self.particles);
        println!("Duration: {}", self.duration);
        println!("Tick: {}", self.tick);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
