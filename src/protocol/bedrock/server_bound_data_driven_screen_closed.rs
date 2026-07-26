use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundDataDrivenScreenClosed {
    pub form_id: u32,
    pub close_reason: String
}

impl Packet for ServerBoundDataDrivenScreenClosed {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundDataDrivenScreenClosed.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_u32_le(self.form_id);
        PacketSerializer::put_string(&mut stream, self.close_reason.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ServerBoundDataDrivenScreenClosed {
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