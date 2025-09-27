use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::sub_chunk_position_offset::SubChunkPositionOffset;

pub struct SubChunkRequest {
    pub dimension: i32,
    pub base_position: Vec<i32>,
    pub entries: Vec<SubChunkPositionOffset>
}

pub fn new(dimension: i32, base_position: Vec<i32>, entries: Vec<SubChunkPositionOffset>) -> SubChunkRequest {
    SubChunkRequest{ dimension, base_position, entries }
}

impl Packet for SubChunkRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSubChunkRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {

        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.dimension);

        PacketSerializer::put_block_pos(&mut stream, self.base_position.clone());

        stream.put_l_int(self.entries.len() as u32);
        for entry in &self.entries {
            entry.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(_bytes: Vec<u8>) -> SubChunkRequest {
        todo!()
    }

    fn debug(&self) {
        println!("Dimension: {}", self.dimension);
        println!("Base Position: {:?}", self.base_position);
        println!("Entries: {:?}", &self.entries);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}