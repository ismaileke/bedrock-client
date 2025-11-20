use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct GameTestResults {
    pub success: bool,
    pub error: String,
    pub test_name: String
}

pub fn new(success: bool, error: String, test_name: String) -> GameTestResults {
    GameTestResults { success, error, test_name }
}

impl Packet for GameTestResults {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGameTestResults.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_bool(self.success);
        PacketSerializer::put_string(&mut stream, self.error.clone());
        PacketSerializer::put_string(&mut stream, self.test_name.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> GameTestResults {
        let success = stream.get_bool();
        let error = PacketSerializer::get_string(stream);
        let test_name = PacketSerializer::get_string(stream);

        GameTestResults { success, error, test_name }
    }

    fn debug(&self) {
        println!("Success: {}", self.success);
        println!("Error: {}", self.error);
        println!("Test Name: {}", self.test_name);   
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
