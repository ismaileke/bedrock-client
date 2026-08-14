use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct GameTestResults {
    pub success: bool,
    pub error: String,
    pub test_name: String,
}

impl Packet for GameTestResults {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGameTestResults.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_bool(self.success);
        PacketSerializer::put_string(stream, &self.error);
        PacketSerializer::put_string(stream, &self.test_name);
    }

    fn decode(stream: &mut Reader) -> GameTestResults {
        let success = stream.get_bool();
        let error = PacketSerializer::get_string(stream);
        let test_name = PacketSerializer::get_string(stream);

        GameTestResults { success, error, test_name }
    }
}
