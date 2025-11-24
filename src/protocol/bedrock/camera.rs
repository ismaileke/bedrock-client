use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct Camera {
    pub camera_actor_unique_id: i64,
    pub player_actor_unique_id: i64
}

pub fn new(camera_actor_unique_id: i64, player_actor_unique_id: i64) -> Camera {
    Camera { camera_actor_unique_id, player_actor_unique_id }
}

impl Packet for Camera {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCamera.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.camera_actor_unique_id);
        PacketSerializer::put_actor_unique_id(&mut stream, self.player_actor_unique_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> Camera {
        let camera_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let player_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);

        Camera { camera_actor_unique_id, player_actor_unique_id }
    }

    fn debug(&self) {
        println!("Camera Actor Unique ID: {}", self.camera_actor_unique_id);
        println!("Player Actor Unique ID: {}", self.player_actor_unique_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
