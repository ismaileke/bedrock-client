use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::enchant_option::EnchantOption;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerEnchantOptions {
    pub options: Vec<EnchantOption>,
}

impl Packet for PlayerEnchantOptions {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerEnchantOptions.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.options.len() as u32);
        for option in &self.options {
            option.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> PlayerEnchantOptions {
        let mut options = Vec::new();
        let len = stream.get_var_u32();
        for _ in 0..len {
            options.push(EnchantOption::read(stream));
        }

        PlayerEnchantOptions { options }
    }
}
