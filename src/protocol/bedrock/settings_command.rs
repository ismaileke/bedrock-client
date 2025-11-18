use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct SettingsCommand {
    pub command: String,
    pub suppress_output: bool
}

pub fn new(command: String, suppress_output: bool) -> SettingsCommand {
    SettingsCommand { command, suppress_output }
}

impl Packet for SettingsCommand {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSettingsCommand.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.command.clone());
        stream.put_bool(self.suppress_output);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> SettingsCommand {
        let mut stream = Stream::new(bytes, 0);

        let command = PacketSerializer::get_string(&mut stream);
        let suppress_output = stream.get_bool();

        SettingsCommand { command, suppress_output }
    }

    fn debug(&self) {
        println!("Command: {}", self.command);
        println!("Suppress Output: {}", self.suppress_output);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
