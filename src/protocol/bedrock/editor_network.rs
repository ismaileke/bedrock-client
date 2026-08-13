use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::tag::tag::Tag;

#[derive(serde::Serialize, Debug)]
pub struct EditorNetwork {
    pub is_route_to_manager: bool,
    pub payload: CacheableNBT,
}

impl Packet for EditorNetwork {
    fn id(&self) -> u16 {
        BedrockPacketType::IDEditorNetwork.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_bool(self.is_route_to_manager);
        stream.put(self.payload.get_encoded_nbt());
    }

    fn decode(stream: &mut Reader) -> EditorNetwork {
        let is_route_to_manager = stream.get_bool();
        let payload = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(stream)));

        EditorNetwork { is_route_to_manager, payload }
    }
}
