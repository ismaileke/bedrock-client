use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct PlayerStartItemCooldown {
    pub item_category: String,
    pub cooldown_ticks: i32
}

pub fn new(item_category: String, cooldown_ticks: i32) -> PlayerStartItemCooldown {
    PlayerStartItemCooldown { item_category, cooldown_ticks }
}

impl Packet for PlayerStartItemCooldown {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerStartItemCooldown.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.item_category.clone());
        stream.put_var_i32(self.cooldown_ticks);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PlayerStartItemCooldown {
        let item_category = PacketSerializer::get_string(stream);
        let cooldown_ticks = stream.get_var_i32();

        PlayerStartItemCooldown { item_category, cooldown_ticks }
    }

    fn debug(&self) {
        println!("Item Category: {}", self.item_category);
        println!("Cooldown Ticks: {}", self.cooldown_ticks);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
