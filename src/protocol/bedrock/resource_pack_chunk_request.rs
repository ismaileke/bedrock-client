use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ResourcePackChunkRequest {
    pub pack_id: String,
    pub chunk_index: u32
}

pub fn new(pack_id: String, chunk_index: u32) -> ResourcePackChunkRequest {
    ResourcePackChunkRequest { pack_id, chunk_index }
}

impl Packet for ResourcePackChunkRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackChunkRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.pack_id.clone());
        stream.put_l_int(self.chunk_index);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ResourcePackChunkRequest {
        let mut stream = Stream::new(bytes, 0);

        let pack_id = PacketSerializer::get_string(&mut stream);
        let chunk_index = stream.get_l_int();

        ResourcePackChunkRequest { pack_id, chunk_index }
    }

    fn debug(&self) {
        println!("Pack ID: {}", self.pack_id);
        println!("Chunk Index: {}", self.chunk_index);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
