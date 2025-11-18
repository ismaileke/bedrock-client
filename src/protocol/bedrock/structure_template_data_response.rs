use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

pub struct StructureTemplateDataResponse {
    pub structure_template_name: String,
    pub nbt: Option<CacheableNBT>,
    pub response_type: u8
}

pub fn new(structure_template_name: String, nbt: Option<CacheableNBT>, response_type: u8) -> StructureTemplateDataResponse {
    StructureTemplateDataResponse { structure_template_name, nbt, response_type }
}

impl Packet for StructureTemplateDataResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDStructureTemplateDataResponse.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.structure_template_name.clone());
        stream.put_bool(self.nbt.is_some());
        if let Some(nbt) = &mut self.nbt {
            stream.put(nbt.get_encoded_nbt());
        }
        stream.put_byte(self.response_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> StructureTemplateDataResponse {
        let mut stream = Stream::new(bytes, 0);

        let structure_template_name = PacketSerializer::get_string(&mut stream);
        let has_nbt = stream.get_bool();
        let mut nbt: Option<CacheableNBT> = None;
        if has_nbt {
            nbt = Some(CacheableNBT::new(Box::new(PacketSerializer::get_nbt_compound_root(&mut stream))));
        }
        let response_type = stream.get_byte();

        StructureTemplateDataResponse { structure_template_name, nbt, response_type }
    }

    fn debug(&self) {
        println!("Structure Template Name: {:?}", self.structure_template_name);
        println!("NBT : {:?}", self.nbt);
        println!("Response Type: {:?}", self.response_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StructureTemplateDataResponse {
    pub const TYPE_FAILURE: u8 = 0;
    pub const TYPE_EXPORT: u8 = 1;
    pub const TYPE_QUERY: u8 = 2;
}
