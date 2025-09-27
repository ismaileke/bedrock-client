use std::any::Any;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;

pub struct NetworkStackLatency {
    pub timestamp: i64,
    pub need_response: bool
}

pub fn new(timestamp: i64, need_response: bool) -> NetworkStackLatency {
    NetworkStackLatency{ timestamp, need_response }
}

pub fn request(timestamp: i64) -> NetworkStackLatency {
    NetworkStackLatency{ timestamp, need_response: true }
}

pub fn response(timestamp: i64) -> NetworkStackLatency {
    NetworkStackLatency{ timestamp, need_response: false }
}

impl Packet for NetworkStackLatency {
    fn id(&self) -> u16 {
        BedrockPacketType::IDNetworkStackLatency.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_long(self.timestamp);
        stream.put_bool(self.need_response);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> NetworkStackLatency {
        let mut stream = Stream::new(bytes, 0);

        let timestamp = stream.get_l_long();
        let need_response = stream.get_bool();

        NetworkStackLatency { timestamp, need_response }
    }

    fn debug(&self) {
        println!("Timestamp: {}", self.timestamp);
        println!("Need Response: {}", self.need_response);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
