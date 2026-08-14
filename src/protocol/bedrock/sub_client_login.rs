use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SubClientLogin {
    pub connection_request_data: String,
}

impl Packet for SubClientLogin {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSubClientLogin.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.connection_request_data);
    }

    fn decode(stream: &mut Reader) -> SubClientLogin {
        let connection_request_data = PacketSerializer::get_string(stream);

        SubClientLogin {
            connection_request_data,
        }
    }
}
