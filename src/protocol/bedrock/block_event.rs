use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct BlockEvent {
    pub block_position: Vec<i32>,
    pub event_type: i32,
    pub event_data: i32
}

pub fn new(block_position: Vec<i32>, event_type: i32, event_data: i32) -> BlockEvent {
    BlockEvent { block_position, event_type, event_data }
}

impl Packet for BlockEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBlockEvent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        stream.put_var_i32(self.event_type);
        stream.put_var_i32(self.event_data);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> BlockEvent {
        let block_position = PacketSerializer::get_block_pos(stream);
        let event_type = stream.get_var_i32();
        let event_data = stream.get_var_i32();

        BlockEvent { block_position, event_type, event_data }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position);
        println!("Event Type: {}", self.event_type);
        println!("Event Data: {}", self.event_data);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
