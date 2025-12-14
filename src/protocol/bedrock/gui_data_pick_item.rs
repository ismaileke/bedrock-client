use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct GUIDataPickItem {
    pub item_description: String,
    pub item_effects: String,
    pub hotbar_slot: i32,
}

pub fn new(item_description: String, item_effects: String, hotbar_slot: i32) -> GUIDataPickItem {
    GUIDataPickItem {
        item_description,
        item_effects,
        hotbar_slot,
    }
}

impl Packet for GUIDataPickItem {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGUIDataPickItem.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.item_description.clone());
        PacketSerializer::put_string(&mut stream, self.item_effects.clone());
        stream.put_i32_le(self.hotbar_slot);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> GUIDataPickItem {
        let item_description = PacketSerializer::get_string(stream);
        let item_effects = PacketSerializer::get_string(stream);
        let hotbar_slot = stream.get_i32_le();

        GUIDataPickItem {
            item_description,
            item_effects,
            hotbar_slot,
        }
    }

    fn debug(&self) {
        println!("Item Description: {}", self.item_description);
        println!("Item Effects: {}", self.item_effects);
        println!("Hotbar Slot: {}", self.hotbar_slot);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
