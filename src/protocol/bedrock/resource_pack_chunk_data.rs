use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct ResourcePackChunkData {
    pub pack_id: String,
    pub chunk_index: u32,
    pub offset: u64,
    pub data: String
}

pub fn new(pack_id: String, chunk_index: u32, offset: u64, data: String) -> ResourcePackChunkData {
    ResourcePackChunkData { pack_id, chunk_index, offset, data }
}

impl Packet for ResourcePackChunkData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackChunkData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.pack_id.clone());
        stream.put_u32_le(self.chunk_index);
        stream.put_u64_le(self.offset);
        PacketSerializer::put_string(&mut stream, self.data.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ResourcePackChunkData {
        let pack_id = PacketSerializer::get_string(stream);
        let chunk_index = stream.get_u32_le();
        let offset = stream.get_u64_le();
        let data = PacketSerializer::get_string(stream);

        ResourcePackChunkData { pack_id, chunk_index, offset, data }
    }

    fn debug(&self) {
        println!("Pack ID: {}", self.pack_id);
        println!("Chunk Index: {}", self.chunk_index);
        println!("Offset: {}", self.offset);
        println!("Data: {}", self.data);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
