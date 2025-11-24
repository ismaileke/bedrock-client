use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct ChangeDimension {
    pub dimension: i32,
    pub position: Vec<f32>,
    pub respawn: bool,
    pub loading_screen_id: Option<u32>
}

pub fn new(dimension: i32, position: Vec<f32>, respawn: bool, loading_screen_id: Option<u32>) -> ChangeDimension {
    ChangeDimension { dimension, position, respawn, loading_screen_id }
}

impl Packet for ChangeDimension {
    fn id(&self) -> u16 {
        BedrockPacketType::IDChangeDimension.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.dimension);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        stream.put_bool(self.respawn);
        PacketSerializer::write_optional(&mut stream, &self.loading_screen_id, |s, v| s.put_u32_le(*v));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ChangeDimension {
        let dimension = stream.get_var_i32();
        let position = PacketSerializer::get_vector3(stream);
        let respawn = stream.get_bool();
        let loading_screen_id = PacketSerializer::read_optional(stream, |s| s.get_u32_le());

        ChangeDimension { dimension, position, respawn, loading_screen_id }
    }

    fn debug(&self) {
        println!("Dimension: {}", self.dimension);
        println!("Position: {:?}", self.position);
        println!("Respawn: {}", self.respawn);
        println!("Loading Screen ID: {:?}", self.loading_screen_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
