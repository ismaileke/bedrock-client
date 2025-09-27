use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct MoveActorAbsolute {
    pub actor_runtime_id: u64,
    pub flags: u8,
    pub position: Vec<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub head_yaw: f32 // always zero for non-mobs
}

pub fn new(actor_runtime_id: u64, flags: u8, position: Vec<f32>, pitch: f32, yaw: f32, head_yaw: f32) -> MoveActorAbsolute {
    MoveActorAbsolute { actor_runtime_id, flags, position, pitch, yaw, head_yaw }
}

impl MoveActorAbsolute {
    pub const FLAG_GROUND: u8 = 0x01;
    pub const FLAG_TELEPORT: u8 = 0x02;
    pub const FLAG_FORCE_MOVE_LOCAL_ENTITY: u8 = 0x04;
}

impl Packet for MoveActorAbsolute {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMoveActorAbsolute.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_byte(self.flags);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        PacketSerializer::put_rotation_byte(&mut stream, self.pitch);
        PacketSerializer::put_rotation_byte(&mut stream, self.yaw);
        PacketSerializer::put_rotation_byte(&mut stream, self.head_yaw);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> MoveActorAbsolute {
        let mut stream = Stream::new(bytes, 0);

        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let flags = stream.get_byte();
        let position = PacketSerializer::get_vector3(&mut stream);
        let pitch = PacketSerializer::get_rotation_byte(&mut stream);
        let yaw = PacketSerializer::get_rotation_byte(&mut stream);
        let head_yaw = PacketSerializer::get_rotation_byte(&mut stream);

        MoveActorAbsolute { actor_runtime_id, flags, position, pitch, yaw, head_yaw }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Flags: {}", self.flags);
        println!("Position: {:?}", self.position);
        println!("Pitch: {}", self.pitch);
        println!("Yaw: {}", self.yaw);
        println!("Head Yaw: {}", self.head_yaw);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
