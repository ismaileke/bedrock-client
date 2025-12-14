use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct OpenSign {
    pub block_position: Vec<i32>,
    pub front: bool,
}

pub fn new(block_position: Vec<i32>, front: bool) -> OpenSign {
    OpenSign {
        block_position,
        front,
    }
}

impl Packet for OpenSign {
    fn id(&self) -> u16 {
        BedrockPacketType::IDOpenSign.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        stream.put_bool(self.front);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> OpenSign {
        let block_position = PacketSerializer::get_block_pos(stream);
        let front = stream.get_bool();

        OpenSign {
            block_position,
            front,
        }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position);
        println!("Front: {}", self.front);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
