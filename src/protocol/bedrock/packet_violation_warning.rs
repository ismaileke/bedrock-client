use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PacketViolationWarning {
    pub violation_type: i32,
    pub severity: i32,
    pub packet_id: i32,
    pub message: String,
}

impl Packet for PacketViolationWarning {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPacketViolationWarning.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.violation_type);
        stream.put_var_i32(self.severity);
        stream.put_var_i32(self.packet_id);
        PacketSerializer::put_string(stream, &self.message);
    }

    fn decode(stream: &mut Reader) -> PacketViolationWarning {
        let violation_type = stream.get_var_i32();
        let severity = stream.get_var_i32();
        let packet_id = stream.get_var_i32();
        let message = PacketSerializer::get_string(stream);

        PacketViolationWarning {
            violation_type,
            severity,
            packet_id,
            message,
        }
    }
}

impl PacketViolationWarning {
    pub const TYPE_MALFORMED: i32 = 0;

    pub const SEVERITY_WARNING: i32 = 0;
    pub const SEVERITY_FINAL_WARNING: i32 = 1;
    pub const SEVERITY_TERMINATING_CONNECTION: i32 = 2;
}
