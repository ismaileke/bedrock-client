use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct MobEffect {
    pub actor_runtime_id: u64,
    pub event_id: u8,
    pub effect_id: i32,
    pub amplifier: i32,
    pub particles: bool,
    pub duration: i32,
    pub tick: u64,
    pub ambient: bool,
}

impl Packet for MobEffect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMobEffect.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_byte(self.event_id);
        stream.put_var_i32(self.effect_id);
        stream.put_var_i32(self.amplifier);
        stream.put_bool(self.particles);
        stream.put_var_i32(self.duration);
        stream.put_var_u64(self.tick);
        stream.put_bool(self.ambient);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> MobEffect {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let event_id = stream.get_byte();
        let effect_id = stream.get_var_i32();
        let amplifier = stream.get_var_i32();
        let particles = stream.get_bool();
        let duration = stream.get_var_i32();
        let tick = stream.get_var_u64();
        let ambient = stream.get_bool();

        MobEffect {
            actor_runtime_id,
            event_id,
            effect_id,
            amplifier,
            particles,
            duration,
            tick,
            ambient,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}

impl MobEffect {
    pub const EVENT_ADD: u8 = 1;
    pub const EVENT_MODIFY: u8 = 2;
    pub const EVENT_REMOVE: u8 = 3;

    pub fn add(
        actor_runtime_id: u64,
        replace: bool,
        effect_id: i32,
        amplifier: i32,
        particles: bool,
        duration: i32,
        tick: u64,
        ambient: bool,
    ) -> MobEffect {
        MobEffect {
            actor_runtime_id,
            event_id: if replace {
            Self::EVENT_MODIFY
            } else {
            Self::EVENT_ADD
            },
            effect_id,
            amplifier,
            particles,
            duration,
            tick,
            ambient,
        }
    }

    pub fn remove(actor_runtime_id: u64, effect_id: i32, tick: u64) -> MobEffect {
        MobEffect {
            actor_runtime_id,
            event_id: Self::EVENT_REMOVE,
            effect_id,
            amplifier: 0,
            particles: false,
            duration: 0,
            tick,
            ambient: false,
        }
    }
}
