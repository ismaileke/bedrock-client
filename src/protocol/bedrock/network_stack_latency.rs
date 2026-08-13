use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

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
        BedrockPacketType::IDNetworkStackLatency.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u64_le(self.timestamp);
        stream.put_bool(self.need_response);
    }

    fn decode(stream: &mut Reader) -> NetworkStackLatency {
        let timestamp = stream.get_u64_le();
        let need_response = stream.get_bool();

        NetworkStackLatency {
            timestamp,
            need_response,
        }
    }
}
