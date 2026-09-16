use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct RecordStarted {
    pub block_position: Vec<i32>,
    pub server_sound_handle: u64,
}

impl Packet for RecordStarted {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRecordStarted.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, &self.block_position);
        stream.put_u64_le(self.server_sound_handle);
    }

    fn decode(stream: &mut Reader) -> RecordStarted {
        let block_position = PacketSerializer::get_block_pos(stream);
        let server_sound_handle = stream.get_u64_le();

        RecordStarted { block_position, server_sound_handle }
    }
}
