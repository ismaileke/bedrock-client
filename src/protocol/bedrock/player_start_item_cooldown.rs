use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerStartItemCooldown {
    pub item_category: String,
    pub cooldown_ticks: i32,
}

impl Packet for PlayerStartItemCooldown {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerStartItemCooldown.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.item_category);
        stream.put_var_i32(self.cooldown_ticks);
    }

    fn decode(stream: &mut Reader) -> PlayerStartItemCooldown {
        let item_category = PacketSerializer::get_string(stream);
        let cooldown_ticks = stream.get_var_i32();

        PlayerStartItemCooldown { item_category, cooldown_ticks }
    }
}
