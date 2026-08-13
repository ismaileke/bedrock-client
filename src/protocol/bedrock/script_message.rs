use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ScriptMessage {
    pub message_id: String,
    pub value: String,
}

impl Packet for ScriptMessage {
    fn id(&self) -> u16 {
        BedrockPacketType::IDScriptMessage.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.message_id.clone());
        PacketSerializer::put_string(stream, self.value.clone());
    }

    fn decode(stream: &mut Reader) -> ScriptMessage {
        let message_id = PacketSerializer::get_string(stream);
        let value = PacketSerializer::get_string(stream);

        ScriptMessage { message_id, value }
    }
}
