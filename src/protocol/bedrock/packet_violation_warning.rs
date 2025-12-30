use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct PacketViolationWarning {
    pub violation_type: i32,
    pub severity: i32,
    pub packet_id: i32,
    pub message: String,
}

impl Packet for PacketViolationWarning {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPacketViolationWarning.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.violation_type);
        stream.put_var_i32(self.severity);
        stream.put_var_i32(self.packet_id);
        PacketSerializer::put_string(&mut stream, self.message.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PacketViolationWarning {
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}

impl PacketViolationWarning {
    pub const TYPE_MALFORMED: i32 = 0;

    pub const SEVERITY_WARNING: i32 = 0;
    pub const SEVERITY_FINAL_WARNING: i32 = 1;
    pub const SEVERITY_TERMINATING_CONNECTION: i32 = 2;
}
