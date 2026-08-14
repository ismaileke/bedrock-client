use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct GameTestRequest {
    pub max_test_per_batch: i32,
    pub repeat_count: i32,
    pub rotation: u8,
    pub stop_on_failure: bool,
    pub test_position: Vec<i32>,
    pub tests_per_row: i32,
    pub test_name: String,
}

impl GameTestRequest {
    pub const ROTATION_0: u8 = 0;
    pub const ROTATION_90: u8 = 1;
    pub const ROTATION_180: u8 = 2;
    pub const ROTATION_270: u8 = 3;
}

impl Packet for GameTestRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGameTestRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.max_test_per_batch);
        stream.put_var_i32(self.repeat_count);
        stream.put_u8(self.rotation);
        stream.put_bool(self.stop_on_failure);
        PacketSerializer::put_block_pos(stream, &self.test_position);
        stream.put_var_i32(self.tests_per_row);
        PacketSerializer::put_string(stream, &self.test_name);
    }

    fn decode(stream: &mut Reader) -> GameTestRequest {
        let max_test_per_batch = stream.get_var_i32();
        let repeat_count = stream.get_var_i32();
        let rotation = stream.get_u8();
        let stop_on_failure = stream.get_bool();
        let test_position = PacketSerializer::get_block_pos(stream);
        let tests_per_row = stream.get_var_i32();
        let test_name = PacketSerializer::get_string(stream);

        GameTestRequest {
            max_test_per_batch,
            repeat_count,
            rotation,
            stop_on_failure,
            test_position,
            tests_per_row,
            test_name,
        }
    }
}
