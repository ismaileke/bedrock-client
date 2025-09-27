use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct Respawn {
    pub position: Vec<f32>,
    pub respawn_state: u8,
    pub actor_runtime_id: u64
}

pub fn new(position: Vec<f32>, respawn_state: u8, actor_runtime_id: u64) -> Respawn {
    Respawn { position, respawn_state, actor_runtime_id }
}

impl Respawn {
    pub const SEARCHING_FOR_SPAWN: u8 = 0;
    pub const READY_TO_SPAWN: u8 = 1;
    pub const CLIENT_READY_TO_SPAWN: u8 = 2;
}

impl Packet for Respawn {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRespawn.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        stream.put_byte(self.respawn_state);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> Respawn {
        let mut stream = Stream::new(bytes, 0);

        let position = PacketSerializer::get_vector3(&mut stream);
        let respawn_state = stream.get_byte();
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);

        Respawn { position, respawn_state, actor_runtime_id }
    }

    fn debug(&self) {
        println!("Position: {:?}", self.position);
        println!("Respawn State: {}", self.respawn_state);
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
