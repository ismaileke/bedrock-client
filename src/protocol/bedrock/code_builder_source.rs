use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CodeBuilderSource {
    pub operation: u8,
    pub category: u8,
    pub code_status: u8,
}

impl Packet for CodeBuilderSource {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCodeBuilderSource.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.operation);
        stream.put_u8(self.category);
        stream.put_u8(self.code_status);
    }

    fn decode(stream: &mut Reader) -> CodeBuilderSource {
        let operation = stream.get_u8();
        let category = stream.get_u8();
        let code_status = stream.get_u8();

        CodeBuilderSource { operation, category, code_status }
    }
}
