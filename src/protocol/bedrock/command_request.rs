use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::command::command_origin_data::CommandOriginData;

pub struct CommandRequest {
    pub command: String,
    pub origin_data: CommandOriginData,
    pub is_internal: bool,
    pub version: i32
}

pub fn new(command: String, origin_data: CommandOriginData, is_internal: bool, version: i32) -> CommandRequest {
    CommandRequest { command, origin_data, is_internal, version }
}

impl Packet for CommandRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCommandRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.command.clone());
        PacketSerializer::put_command_origin_data(&mut stream, &self.origin_data);
        stream.put_bool(self.is_internal);
        stream.put_var_i32(self.version);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CommandRequest {
        let command = PacketSerializer::get_string(stream);
        let origin_data = PacketSerializer::get_command_origin_data(stream);
        let is_internal = stream.get_bool();
        let version = stream.get_var_i32();

        CommandRequest { command, origin_data, is_internal, version }
    }

    fn debug(&self) {
        println!("Command: {}", self.command);
        println!("Origin Data: {:?}", self.origin_data);
        println!("Is Internal: {}", self.is_internal);
        println!("Version: {}", self.version);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
