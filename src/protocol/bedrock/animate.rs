use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct Animate {
    pub action: i32,
    pub actor_runtime_id: u64,
    pub data: f32,
    pub rowing_time: f32
}

pub fn new(action: i32, actor_runtime_id: u64, data: f32) -> Animate {
    Animate{ action, actor_runtime_id, data, rowing_time: 0.0 }
}

pub fn boat_hack(action: i32, actor_runtime_id: u64, rowing_time: f32) -> Animate {
    if action != Animate::ACTION_ROW_LEFT && action != Animate::ACTION_ROW_RIGHT {
        panic!("Invalid actionId for boatHack: {}", action);
    }
    let mut animate = new(action, actor_runtime_id, 0.0);
    animate.rowing_time = rowing_time;

    animate
}

impl Packet for Animate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAnimate.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {

        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.action);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_f32_le(self.data);
        if self.action == Animate::ACTION_ROW_RIGHT || self.action == Animate::ACTION_ROW_LEFT {
            stream.put_f32_le(self.rowing_time);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> Animate {
        let action = stream.get_var_i32();
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let data = stream.get_f32_le();
        let mut rowing_time = 0.0;
        if action == Animate::ACTION_ROW_LEFT || action == Animate::ACTION_ROW_RIGHT {
            rowing_time = stream.get_f32_le();
        }

        Animate { action, actor_runtime_id, data, rowing_time }
    }

    fn debug(&self) {
        println!("Action: {}", self.action);
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Data: {}", self.data);
        println!("Rowing time: {}", self.rowing_time);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Animate {
    pub const ACTION_SWING_ARM: i32 = 1;
    pub const ACTION_STOP_SLEEP: i32 = 3;
    pub const ACTION_CRITICAL_HIT: i32 = 4;
    pub const ACTION_MAGICAL_CRITICAL_HIT: i32 = 5;
    pub const ACTION_ROW_RIGHT: i32 = 128;
    pub const ACTION_ROW_LEFT: i32 = 129;
}
