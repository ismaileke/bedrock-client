use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::enchant_option::EnchantOption;

#[derive(serde::Serialize, Debug)]
pub struct PlayerEnchantOptions {
    pub options: Vec<EnchantOption>
}

pub fn new(options: Vec<EnchantOption>) -> PlayerEnchantOptions {
    PlayerEnchantOptions { options }
}

impl Packet for PlayerEnchantOptions {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerEnchantOptions.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.options.len() as u32);
        for option in &self.options {
            option.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PlayerEnchantOptions {
        let mut options = Vec::new();
        let len = stream.get_var_u32();
        for _ in 0..len {
            options.push(EnchantOption::read(stream));
        }

        PlayerEnchantOptions { options }
    }

    fn debug(&self) {
        println!("Player Enchant Options: {:?}", self.options);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
