use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ResourcePackChunkRequest {
    pub pack_id: String,
    pub chunk_index: u32,
}

impl Packet for ResourcePackChunkRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackChunkRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.pack_id.clone());
        stream.put_u32_le(self.chunk_index);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ResourcePackChunkRequest {
        let pack_id = PacketSerializer::get_string(stream);
        let chunk_index = stream.get_u32_le();

        ResourcePackChunkRequest { pack_id, chunk_index }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
