use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct PositionTrackingDBClientRequest {
    pub action: u8,
    pub tracking_id: i32
}

pub fn new(action: u8, tracking_id: i32) -> PositionTrackingDBClientRequest {
    PositionTrackingDBClientRequest { action, tracking_id }
}

impl PositionTrackingDBClientRequest {
    pub const ACTION_QUERY: u8 = 0;
}

impl Packet for PositionTrackingDBClientRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPositionTrackingDBClientRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_byte(self.action);
        stream.put_var_int(self.tracking_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PositionTrackingDBClientRequest {
        let mut stream = Stream::new(bytes, 0);

        let action = stream.get_byte();
        let tracking_id = stream.get_var_int();

        PositionTrackingDBClientRequest { action, tracking_id }
    }

    fn debug(&self) {
        println!("Action: {}", self.action);
        println!("Tracking ID: {}", self.tracking_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
