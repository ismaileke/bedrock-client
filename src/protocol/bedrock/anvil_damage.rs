use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct AnvilDamage {
    pub damage_amount: u8,
    pub block_pos: Vec<i32>
}

pub fn new(damage_amount: u8, block_pos: Vec<i32>) -> AnvilDamage {
    AnvilDamage { damage_amount, block_pos }
}

impl Packet for AnvilDamage {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAnvilDamage.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.damage_amount);
        PacketSerializer::put_block_pos(&mut stream, self.block_pos.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> AnvilDamage {
        let damage_amount = stream.get_byte();
        let block_pos = PacketSerializer::get_block_pos(stream);

        AnvilDamage { damage_amount, block_pos }
    }

    fn debug(&self) {
        println!("Damage Amount: {}", self.damage_amount);
        println!("Block Pos: {:?}", self.block_pos);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
