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
        stream.put_var_u32(self.id() as u32);

        stream.put_u32_le(self.effect_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> OnScreenTextureAnimation {
        let effect_id = stream.get_u32_le();

        OnScreenTextureAnimation { effect_id }
    }

    fn debug(&self) {
        println!("Effect ID: {}", self.effect_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
