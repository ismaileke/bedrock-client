use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct NetworkStackLatency {
    pub timestamp: u64,
    pub need_response: bool,
}

impl NetworkStackLatency {
    pub fn request(timestamp: u64) -> NetworkStackLatency {
        NetworkStackLatency {
            timestamp,
            need_response: true,
        }
    }

    pub fn response(timestamp: u64) -> NetworkStackLatency {
        NetworkStackLatency {
            timestamp,
            need_response: false,
        }
    }
}

impl Packet for NetworkStackLatency {
    fn id(&self) -> u16 {
        BedrockPacketType::IDNetworkStackLatency.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_u64_le(self.timestamp);
        stream.put_bool(self.need_response);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> NetworkStackLatency {
        let timestamp = stream.get_u64_le();
        let need_response = stream.get_bool();

        NetworkStackLatency {
            timestamp,
            need_response,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
