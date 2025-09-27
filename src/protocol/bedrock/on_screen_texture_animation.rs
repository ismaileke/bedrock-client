use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct OnScreenTextureAnimation {
    pub effect_id: u32
}

pub fn new(effect_id: u32) -> OnScreenTextureAnimation {
    OnScreenTextureAnimation { effect_id }
}

impl Packet for OnScreenTextureAnimation {
    fn id(&self) -> u16 {
        BedrockPacketType::IDOnScreenTextureAnimation.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_int(self.effect_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> OnScreenTextureAnimation {
        let mut stream = Stream::new(bytes, 0);

        let effect_id = stream.get_l_int();

        OnScreenTextureAnimation { effect_id }
    }

    fn debug(&self) {
        println!("Effect ID: {}", self.effect_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
