use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ResourcePackDataInfo {
    pub pack_id: String,
    pub max_chunk_size: u32,
    pub chunk_count: u32,
    pub compressed_pack_size: u64,
    pub sha256: String,
    pub is_premium: bool,
    pub pack_type: u8, // see types/resource_pack_type
}

impl Packet for ResourcePackDataInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackDataInfo.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.pack_id.clone());
        stream.put_u32_le(self.max_chunk_size);
        stream.put_u32_le(self.chunk_count);
        stream.put_u64_le(self.compressed_pack_size);
        PacketSerializer::put_string(&mut stream, self.sha256.clone());
        stream.put_bool(self.is_premium);
        stream.put_byte(self.pack_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ResourcePackDataInfo {
        let pack_id = PacketSerializer::get_string(stream);
        let max_chunk_size = stream.get_u32_le();
        let chunk_count = stream.get_u32_le();
        let compressed_pack_size = stream.get_u64_le();
        let sha256 = PacketSerializer::get_string(stream);
        let is_premium = stream.get_bool();
        let pack_type = stream.get_byte();

        ResourcePackDataInfo {
            pack_id,
            max_chunk_size,
            chunk_count,
            compressed_pack_size,
            sha256,
            is_premium,
            pack_type,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
