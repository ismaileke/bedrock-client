use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;

pub const ACTION_SWING_ARM: i32 = 1;
pub const ACTION_STOP_SLEEP: i32 = 3;
pub const ACTION_CRITICAL_HIT: i32 = 4;
pub const ACTION_MAGICAL_CRITICAL_HIT: i32 = 5;

pub struct Animate {
    action: i32,
    actor_runtime_id: u64,
    float: f32
}

pub fn new(action: i32, actor_runtime_id: u64) -> Animate {
    Animate{ action, actor_runtime_id, float: 0.0 }
}

pub fn boat_hack(action: i32, actor_runtime_id: u64, float: f32) -> Animate {
    Animate{ action, actor_runtime_id, float }
}

impl Animate {
    pub fn encode(&mut self) -> Vec<u8> {

        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::Animate) as u32);

        stream.put_var_int(self.action);
        stream.put_unsigned_var_long(self.actor_runtime_id);
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

    pub fn debug(&self) {
        println!("Action: {}", self.action);
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Float: {}", self.float);
    }
}