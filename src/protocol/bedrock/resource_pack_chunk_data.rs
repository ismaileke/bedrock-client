use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ResourcePackChunkData {
    pub pack_id: String,
    pub chunk_index: u32,
    pub offset: u64,
    pub data: Vec<u8>,
}

impl Packet for ResourcePackChunkData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackChunkData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.pack_id);
        stream.put_u32_le(self.chunk_index);
        stream.put_u64_le(self.offset);
        PacketSerializer::put_byte_string(stream, &self.data);
    }

    fn decode(stream: &mut Reader) -> ResourcePackChunkData {
        let pack_id = PacketSerializer::get_string(stream);
        let chunk_index = stream.get_u32_le();
        let offset = stream.get_u64_le();
        let data = PacketSerializer::get_byte_string(stream);

        ResourcePackChunkData { pack_id, chunk_index, offset, data }
    }
}
