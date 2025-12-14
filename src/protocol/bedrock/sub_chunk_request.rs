use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::sub_chunk_position_offset::SubChunkPositionOffset;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct SubChunkRequest {
    pub dimension: i32,
    pub base_position: Vec<i32>,
    pub entries: Vec<SubChunkPositionOffset>,
}

pub fn new(
    dimension: i32,
    base_position: Vec<i32>,
    entries: Vec<SubChunkPositionOffset>,
) -> SubChunkRequest {
    SubChunkRequest {
        dimension,
        base_position,
        entries,
    }
}

impl Packet for SubChunkRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSubChunkRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.dimension);

        PacketSerializer::put_block_pos(&mut stream, self.base_position.clone());

        stream.put_u32_le(self.entries.len() as u32);
        for entry in &self.entries {
            entry.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(_stream: &mut Stream) -> SubChunkRequest {
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

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
