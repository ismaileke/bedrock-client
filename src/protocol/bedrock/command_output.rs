use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::command::command_output_message::CommandOutputMessage;
use crate::protocol::bedrock::types::command::command_origin_data::CommandOriginData;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CommandOutput {
    origin_data: CommandOriginData,
    output_type: String,
    success_count: u32,
    messages: Vec<CommandOutputMessage>,
    data: Option<String>
}

impl Packet for CommandOutput {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCommandOutput.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_command_origin_data(&mut stream, &self.origin_data);
        PacketSerializer::put_string(&mut stream, self.output_type.clone());
        stream.put_u32_le(self.success_count);
        stream.put_var_u32(self.messages.len() as u32);
        for message in &self.messages {
            Self::put_command_message(message, &mut stream);
        }
        PacketSerializer::write_optional(&mut stream, &self.data, |s, v| PacketSerializer::put_string(s, v.clone()));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CommandOutput {
        let origin_data = PacketSerializer::get_command_origin_data(stream);
        let output_type = PacketSerializer::get_string(stream);
        let success_count = stream.get_u32_le();
        let size = stream.get_var_u32();
        let mut messages = Vec::new();
        for _ in 0..size {
            messages.push(Self::get_command_message(stream));
        }
        let data = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));

        CommandOutput { origin_data, output_type, success_count, messages, data }
    }
}

impl CommandOutput {
    pub const TYPE_LAST: &'static str = "lastoutput";
    pub const TYPE_SILENT: &'static str = "silent";
    pub const TYPE_ALL: &'static str = "alloutput";
    pub const TYPE_DATA_SET: &'static str = "dataset";

    fn get_command_message(stream: &mut Stream) -> CommandOutputMessage {
        let message_id = PacketSerializer::get_string(stream);
        let is_internal = stream.get_bool();
        let size = stream.get_var_u32();
        let mut parameters = Vec::new();
        for _ in 0..size {
            parameters.push(PacketSerializer::get_string(stream));
        }
        CommandOutputMessage { message_id, is_internal, parameters }
    }
    fn put_command_message(message: &CommandOutputMessage, stream: &mut Stream) {
        PacketSerializer::put_string(stream, message.message_id.clone());
        stream.put_bool(message.is_internal);
        stream.put_var_u32(message.parameters.len() as u32);
        for parameter in &message.parameters {
            PacketSerializer::put_string(stream, parameter.to_string());
        }
    }
}