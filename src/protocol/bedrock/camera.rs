use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

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
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.camera_actor_unique_id);
        PacketSerializer::put_actor_unique_id(&mut stream, self.player_actor_unique_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> Camera {
        let mut stream = Stream::new(bytes, 0);

        let camera_actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);
        let player_actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);

        Camera { camera_actor_unique_id, player_actor_unique_id }
    }

    fn debug(&self) {
        println!("Camera Actor Unique ID: {}", self.camera_actor_unique_id);
        println!("Player Actor Unique ID: {}", self.player_actor_unique_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
