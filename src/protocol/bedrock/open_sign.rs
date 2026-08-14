use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct OpenSign {
    pub block_position: Vec<i32>,
    pub front: bool,
}

impl Packet for OpenSign {
    fn id(&self) -> u16 {
        BedrockPacketType::IDOpenSign.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, &self.block_position);
        stream.put_bool(self.front);
    }

    fn decode(stream: &mut Reader) -> OpenSign {
        let block_position = PacketSerializer::get_block_pos(stream);
        let front = stream.get_bool();

        OpenSign {
            block_position,
            front,
        }
    }
}
