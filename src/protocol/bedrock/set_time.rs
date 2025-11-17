use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct SetTime {
    pub time: i32
}

pub fn new(time: i32) -> SetTime {
    SetTime { time }
}

impl Packet for SetTime {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetTime.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.time);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> SetTime {
        let mut stream = Stream::new(bytes, 0);

        let time = stream.get_var_i32();

        SetTime { time }
    }

    fn debug(&self) {
        println!("Time: {}", self.time);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
