use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::tag::tag::Tag;

#[derive(serde::Serialize, Debug)]
pub struct StructureTemplateDataResponse {
    pub structure_template_name: String,
    pub nbt: Option<CacheableNBT>,
    pub response_type: u8,
}

impl Packet for StructureTemplateDataResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDStructureTemplateDataResponse.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.structure_template_name);
        stream.put_bool(self.nbt.is_some());
        if let Some(nbt) = &mut self.nbt {
            stream.put(nbt.get_encoded_nbt());
        }
        stream.put_u8(self.response_type);
    }

    fn decode(stream: &mut Reader) -> StructureTemplateDataResponse {
        let structure_template_name = PacketSerializer::get_string(stream);
        let has_nbt = stream.get_bool();
        let mut nbt: Option<CacheableNBT> = None;
        if has_nbt {
            nbt = Some(CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(stream))));
        }
        let response_type = stream.get_u8();

        StructureTemplateDataResponse {
            structure_template_name,
            nbt,
            response_type,
        }
    }
}

impl StructureTemplateDataResponse {
    pub const TYPE_FAILURE: u8 = 0;
    pub const TYPE_EXPORT: u8 = 1;
    pub const TYPE_QUERY: u8 = 2;
}
