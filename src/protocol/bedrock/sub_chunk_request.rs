use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::sub_chunk_position_offset::SubChunkPositionOffset;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::sub_chunk_position::SubChunkPosition;

#[derive(serde::Serialize, Debug)]
pub struct SubChunkRequest {
    pub dimension: i32,
    pub base_position: SubChunkPosition,
    pub entries: Vec<SubChunkPositionOffset>,
}

impl Packet for SubChunkRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSubChunkRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.dimension);
        stream.put_var_u32(self.entries.len() as u32);
        for entry in &self.entries {
            entry.write(&mut stream);
        }
        self.base_position.write_fixed_ints(&mut stream);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SubChunkRequest {
        let dimension = stream.get_var_i32();
        let len = stream.get_var_u32() as usize;
        let mut entries = vec![];
        for _ in 0..len {
            entries.push(SubChunkPositionOffset::read(stream));
        }
        let base_position = SubChunkPosition::read_fixed_ints(stream);

        SubChunkRequest { dimension, base_position, entries }
    }
}
