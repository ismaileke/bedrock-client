use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CodeBuilder {
    pub url: String,
    pub open_code_builder: bool,
}

impl Packet for CodeBuilder {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCodeBuilder.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.url);
        stream.put_bool(self.open_code_builder);
    }

    fn decode(stream: &mut Reader) -> CodeBuilder {
        let url = PacketSerializer::get_string(stream);
        let open_code_builder = stream.get_bool();

        CodeBuilder { url, open_code_builder }
    }
}
