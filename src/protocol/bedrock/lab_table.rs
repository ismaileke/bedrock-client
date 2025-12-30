use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct LabTable {
    pub action_type: u8,
    pub block_position: Vec<i32>,
    pub reaction_type: u8,
}

impl Packet for LabTable {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLabTable.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.action_type);
        PacketSerializer::put_signed_block_pos(&mut stream, self.block_position.clone());
        stream.put_byte(self.reaction_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> LabTable {
        let action_type = stream.get_byte();
        let block_position = PacketSerializer::get_signed_block_pos(stream);
        let reaction_type = stream.get_byte();

        LabTable {
            action_type,
            block_position,
            reaction_type,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}

impl LabTable {
    pub const TYPE_START_COMBINE: u8 = 0;
    pub const TYPE_START_REACTION: u8 = 1;
    pub const TYPE_RESET: u8 = 2;
}
