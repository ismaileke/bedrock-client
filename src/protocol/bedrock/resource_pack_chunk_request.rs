use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ResourcePackChunkRequest {
    pub pack_id: String,
    pub chunk_index: u32,
}

impl Packet for ResourcePackChunkRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackChunkRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.pack_id);
        stream.put_u32_le(self.chunk_index);
    }

    fn decode(stream: &mut Reader) -> ResourcePackChunkRequest {
        let pack_id = PacketSerializer::get_string(stream);
        let chunk_index = stream.get_u32_le();

        ResourcePackChunkRequest { pack_id, chunk_index }
    }
}
