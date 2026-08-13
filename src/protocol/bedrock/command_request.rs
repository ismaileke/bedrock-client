use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::command::command_origin_data::CommandOriginData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CommandRequest {
    pub command: String,
    pub origin_data: CommandOriginData,
    pub is_internal: bool,
    pub version: String,
}

impl Packet for CommandRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCommandRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.command.clone());
        PacketSerializer::put_command_origin_data(stream, &self.origin_data);
        stream.put_bool(self.is_internal);
        PacketSerializer::put_string(stream, self.version.clone());
    }

    fn decode(stream: &mut Reader) -> CommandRequest {
        let command = PacketSerializer::get_string(stream);
        let origin_data = PacketSerializer::get_command_origin_data(stream);
        let is_internal = stream.get_bool();
        let version = PacketSerializer::get_string(stream);

        CommandRequest { command, origin_data, is_internal, version }
    }
}
