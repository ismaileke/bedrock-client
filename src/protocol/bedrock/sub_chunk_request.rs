use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::sub_chunk_position_offset::SubChunkPositionOffset;

pub struct SubChunkRequest {
    dimension: i32,
    base_position: Vec<i32>,
    entries: Vec<SubChunkPositionOffset>
}

pub fn new(dimension: i32, base_position: Vec<i32>, entries: Vec<SubChunkPositionOffset>) -> SubChunkRequest {
    SubChunkRequest{ dimension, base_position, entries }
}

impl SubChunkRequest {
    pub fn encode(&mut self) -> Vec<u8> {

        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::SubChunkRequest) as u32);

        stream.put_var_int(self.dimension);
        stream.put_var_int(self.base_position[0]);
        stream.put_var_int(self.base_position[1]);
        stream.put_var_int(self.base_position[2]);

        stream.put_l_int(self.entries.len() as u32);
        for entry in &self.entries {
            entry.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    pub fn debug(&self) {
        println!("Dimension: {}", self.dimension);
        println!("Base Position: {:?}", self.base_position);
        println!("Entries: {:?}", &self.entries);
    }
}