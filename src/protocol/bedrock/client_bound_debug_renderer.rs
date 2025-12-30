use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::debug_marker_data::DebugMarkerData;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundDebugRenderer {
    pub debug_type: String,
    pub debug_marker_data: Option<DebugMarkerData>,
}

impl ClientBoundDebugRenderer {
    pub const TYPE_CLEAR: &'static str = "cleardebugmarkers";
    pub const TYPE_ADD_CUBE: &'static str = "cleardebugmarkers";
}

impl Packet for ClientBoundDebugRenderer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundDebugRenderer.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.debug_type.to_string());
        PacketSerializer::write_optional(&mut stream, &self.debug_marker_data, |s, v| v.write(s));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ClientBoundDebugRenderer {
        let debug_type = PacketSerializer::get_string(stream);
        let debug_marker_data =
            PacketSerializer::read_optional(stream, |s| DebugMarkerData::read(s));

        ClientBoundDebugRenderer { debug_type, debug_marker_data }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
