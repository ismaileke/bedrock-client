use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct GameTestRequest {
    pub max_test_per_batch: i32,
    pub repeat_count: i32,
    pub rotation: u8,
    pub stop_on_failure: bool,
    pub test_position: Vec<i32>,
    pub tests_per_row: i32,
    pub test_name: String
}

pub fn new(max_test_per_batch: i32, repeat_count: i32, rotation: u8, stop_on_failure: bool, test_position: Vec<i32>, tests_per_row: i32, test_name: String) -> GameTestRequest {
    GameTestRequest { max_test_per_batch, repeat_count, rotation, stop_on_failure, test_position, tests_per_row, test_name }
}

impl Packet for GameTestRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGameTestRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.max_test_per_batch);
        stream.put_var_i32(self.repeat_count);
        stream.put_byte(self.rotation);
        stream.put_bool(self.stop_on_failure);
        PacketSerializer::put_signed_block_pos(&mut stream, self.test_position.clone());
        stream.put_var_i32(self.tests_per_row);
        PacketSerializer::put_string(&mut stream, self.test_name.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> GameTestRequest {
        let max_test_per_batch = stream.get_var_i32();
        let repeat_count = stream.get_var_i32();
        let rotation = stream.get_byte();
        let stop_on_failure = stream.get_bool();
        let test_position = PacketSerializer::get_signed_block_pos(stream);
        let tests_per_row = stream.get_var_i32();
        let test_name = PacketSerializer::get_string(stream);

        GameTestRequest { max_test_per_batch, repeat_count, rotation, stop_on_failure, test_position, tests_per_row, test_name }
    }

    fn debug(&self) {
        println!("Max Test Per Batch: {}", self.max_test_per_batch);
        println!("Repeat Count: {}", self.repeat_count);
        println!("Rotation: {}", self.rotation);
        println!("Stop On Failure: {}", self.stop_on_failure);
        println!("Test Position: {:?}", self.test_position);
        println!("Tests Per Row: {}", self.tests_per_row);
        println!("Test Name: {}", self.test_name);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl GameTestRequest {
    pub const ROTATION_0: u8 = 0;
    pub const ROTATION_90: u8 = 1;
    pub const ROTATION_180: u8 = 2;
    pub const ROTATION_270: u8 = 3;
}
