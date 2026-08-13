use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AnvilDamage {
    pub damage_amount: u8,
    pub block_pos: Vec<i32>,
}

impl Packet for AnvilDamage {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAnvilDamage.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.damage_amount);
        PacketSerializer::put_block_pos(stream, self.block_pos.clone());
    }

    fn decode(stream: &mut Reader) -> AnvilDamage {
        let damage_amount = stream.get_u8();
        let block_pos = PacketSerializer::get_block_pos(stream);

        AnvilDamage { damage_amount, block_pos }
    }
}
