use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ToastRequest {
    pub title: String,
    pub body: String,
}

impl Packet for ToastRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDToastRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.title.clone());
        PacketSerializer::put_string(stream, self.body.clone());
    }

    fn decode(stream: &mut Reader) -> ToastRequest {
        let title = PacketSerializer::get_string(stream);
        let body = PacketSerializer::get_string(stream);

        ToastRequest { title, body }
    }
}
