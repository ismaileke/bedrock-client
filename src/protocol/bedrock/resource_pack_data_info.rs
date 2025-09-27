use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ResourcePackDataInfo {
    pub pack_id: String,
    pub max_chunk_size: u32,
    pub chunk_count: u32,
    pub compressed_pack_size: i64,
    pub sha256: String,
    pub is_premium: bool,
    pub pack_type: u8 // see types/resource_pack_type
}

pub fn new(pack_id: String, max_chunk_size: u32, chunk_count: u32, compressed_pack_size: i64, sha256: String, is_premium: bool, pack_type: u8) -> ResourcePackDataInfo {
    ResourcePackDataInfo { pack_id, max_chunk_size, chunk_count, compressed_pack_size, sha256, is_premium, pack_type }
}

impl Packet for ResourcePackDataInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackDataInfo.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.pack_id.clone());
        stream.put_l_int(self.max_chunk_size);
        stream.put_l_int(self.chunk_count);
        stream.put_l_long(self.compressed_pack_size);
        PacketSerializer::put_string(&mut stream, self.sha256.clone());
        stream.put_bool(self.is_premium);
        stream.put_byte(self.pack_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ResourcePackDataInfo {
        let mut stream = Stream::new(bytes, 0);

        let pack_id = PacketSerializer::get_string(&mut stream);
        let max_chunk_size = stream.get_l_int();
        let chunk_count = stream.get_l_int();
        let compressed_pack_size = stream.get_l_long();
        let sha256 = PacketSerializer::get_string(&mut stream);
        let is_premium = stream.get_bool();
        let pack_type = stream.get_byte();

        ResourcePackDataInfo { pack_id, max_chunk_size, chunk_count, compressed_pack_size, sha256, is_premium, pack_type }
    }

    fn debug(&self) {
        println!("Pack ID: {}", self.pack_id);
        println!("Max Chunk Size: {}", self.max_chunk_size);
        println!("Chunk Count: {}", self.chunk_count);
        println!("Compressed Pack Size: {}", self.compressed_pack_size);
        println!("SHA256: {}", self.sha256);
        println!("Is Premium: {}", self.is_premium);
        println!("Pack Type: {}", self.pack_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
