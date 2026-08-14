use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::debug_marker_data::DebugMarkerData;
use binary_utils::binary::{Reader, Writer};

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
        BedrockPacketType::IDClientBoundDebugRenderer.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.debug_type);
        PacketSerializer::write_optional(stream, &self.debug_marker_data, |s, v| v.write(s));
    }

    fn decode(stream: &mut Reader) -> ClientBoundDebugRenderer {
        let debug_type = PacketSerializer::get_string(stream);
        let debug_marker_data =
            PacketSerializer::read_optional(stream, |s| DebugMarkerData::read(s));

        ClientBoundDebugRenderer { debug_type, debug_marker_data }
    }
}
