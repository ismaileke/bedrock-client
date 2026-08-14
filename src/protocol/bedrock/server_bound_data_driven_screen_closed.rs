use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundDataDrivenScreenClosed {
    pub form_id: u32,
    pub close_reason: String
}

impl Packet for ServerBoundDataDrivenScreenClosed {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundDataDrivenScreenClosed.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u32_le(self.form_id);
        PacketSerializer::put_string(stream, &self.close_reason);
    }

    fn decode(stream: &mut Reader) -> ServerBoundDataDrivenScreenClosed {
        let form_id = stream.get_u32_le();
        let close_reason = PacketSerializer::get_string(stream);

        ServerBoundDataDrivenScreenClosed { form_id, close_reason }
    }
}

impl ServerBoundDataDrivenScreenClosed {
    pub const PROGRAMMATIC_CLOSE: &str = "programmaticclose";
    pub const PROGRAMMATIC_CLOSE_ALL: &str = "programmaticcloseall";
    pub const CLIENT_CANCELED: &str = "clientcanceled";
    pub const USER_BUSY: &str = "userbusy";
    pub const INVALID_FORM: &str = "invalidform";
}