use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct Animate {
    pub action: i32,
    pub actor_runtime_id: u64,
    pub float: f32
}

pub fn new(action: i32, actor_runtime_id: u64) -> Animate {
    Animate{ action, actor_runtime_id, float: 0.0 }
}

pub fn boat_hack(action: i32, actor_runtime_id: u64, float: f32) -> Animate {
    Animate{ action, actor_runtime_id, float }
}

impl Animate {
    pub const ACTION_SWING_ARM: i32 = 1;
    pub const ACTION_STOP_SLEEP: i32 = 3;
    pub const ACTION_CRITICAL_HIT: i32 = 4;
    pub const ACTION_MAGICAL_CRITICAL_HIT: i32 = 5;
}

impl Packet for Animate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAnimate.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {

        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.action);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        if self.action & 0x80 != 0 {
            stream.put_l_float(self.float);
        } else {
            stream.put_l_float(0.0);        // probably
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> Animate {
        let mut stream = Stream::new(bytes, 0);

        let action = stream.get_var_int();
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let float = stream.get_l_float();

        Animate { action, actor_runtime_id, float }
    }

    fn debug(&self) {
        println!("Action: {}", self.action);
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Float: {}", self.float);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
