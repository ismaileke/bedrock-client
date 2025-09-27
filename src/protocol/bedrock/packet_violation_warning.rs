use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct PacketViolationWarning {
    pub violation_type: i32,
    pub severity: i32,
    pub packet_id: i32,
    pub message: String
}

pub fn new(violation_type: i32, severity: i32, packet_id: i32, message: String) -> PacketViolationWarning {
    PacketViolationWarning { violation_type, severity, packet_id, message }
}

impl PacketViolationWarning {
    pub const TYPE_MALFORMED: i32 = 0;

    pub const SEVERITY_WARNING: i32 = 0;
    pub const SEVERITY_FINAL_WARNING: i32 = 1;
    pub const SEVERITY_TERMINATING_CONNECTION: i32 = 2;
}

impl Packet for PacketViolationWarning {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPacketViolationWarning.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.violation_type);
        stream.put_var_int(self.severity);
        stream.put_var_int(self.packet_id);
        PacketSerializer::put_string(&mut stream, self.message.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PacketViolationWarning {
        let mut stream = Stream::new(bytes, 0);

        let violation_type = stream.get_var_int();
        let severity = stream.get_var_int();
        let packet_id = stream.get_var_int();
        let message = PacketSerializer::get_string(&mut stream);

        PacketViolationWarning { violation_type, severity, packet_id, message }
    }

    fn debug(&self) {
        println!("Violation Type: {}", self.violation_type);
        println!("Severity: {}", self.severity);
        println!("Packet ID: {}", self.packet_id);
        println!("Message: {}", self.message);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
