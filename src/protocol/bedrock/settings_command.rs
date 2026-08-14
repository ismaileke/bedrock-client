use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SettingsCommand {
    pub command: String,
    pub suppress_output: bool,
}

impl Packet for SettingsCommand {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSettingsCommand.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.command);
        stream.put_bool(self.suppress_output);
    }

    fn decode(stream: &mut Reader) -> SettingsCommand {
        let command = PacketSerializer::get_string(stream);
        let suppress_output = stream.get_bool();

        SettingsCommand { command, suppress_output }
    }
}
