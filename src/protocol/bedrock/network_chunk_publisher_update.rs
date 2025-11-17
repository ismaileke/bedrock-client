use std::any::Any;
use binary_utils::binary::Stream;
use log::error;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

const MAX_SAVED_CHUNKS: u32 = 9216;

pub struct NetworkChunkPublisherUpdate {
    pub block_pos: Vec<i32>,
    pub radius: u32,
    pub saved_chunks: Vec<Vec<i32>>
}

impl Packet for NetworkChunkPublisherUpdate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDNetworkChunkPublisherUpdate.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_block_pos(&mut stream, self.block_pos.clone());

        stream.put_var_u32(self.radius);

        stream.put_u32_le(self.saved_chunks.len() as u32);
        for chunk in self.saved_chunks.iter() {
            stream.put_var_i32(chunk[0]);
            stream.put_var_i32(chunk[1]);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> NetworkChunkPublisherUpdate {
        let mut stream = Stream::new(bytes, 0);

        let block_pos = PacketSerializer::get_block_pos(&mut stream);

        let radius = stream.get_var_u32();

        let count = stream.get_u32_le();

        if count > MAX_SAVED_CHUNKS {
            error!("Expected at most {} saved chunks, got {}", MAX_SAVED_CHUNKS, count)
        }

        let mut saved_chunks = vec![];
        for _ in 0..count {
            let chunk_x = stream.get_var_i32();
            let chunk_z = stream.get_var_i32();
            saved_chunks.push(vec![chunk_x, chunk_z]);
        }

        NetworkChunkPublisherUpdate { block_pos, radius, saved_chunks }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_pos);
        println!("Radius: {}", self.radius);
        println!("Saved Chunks: {:?}", self.saved_chunks);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
