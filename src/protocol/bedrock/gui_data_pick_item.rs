use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct GUIDataPickItem {
    pub item_description: String,
    pub item_effects: String,
    pub hotbar_slot: i32,
}

impl Packet for GUIDataPickItem {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGUIDataPickItem.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.item_description);
        PacketSerializer::put_string(stream, &self.item_effects);
        stream.put_i32_le(self.hotbar_slot);
    }

    fn decode(stream: &mut Reader) -> GUIDataPickItem {
        let item_description = PacketSerializer::get_string(stream);
        let item_effects = PacketSerializer::get_string(stream);
        let hotbar_slot = stream.get_i32_le();

        GUIDataPickItem { item_description, item_effects, hotbar_slot }
    }
}
