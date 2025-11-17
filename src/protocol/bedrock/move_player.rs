use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct MovePlayer {
    pub actor_runtime_id: u64,
    pub flags: u8,
    pub position: Vec<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub head_yaw: f32,
    pub mode: u8,
    pub on_ground: bool,
    pub riding_actor_runtime_id: u64,
    pub teleport_cause: i32,
    pub teleport_item: i32,
    pub tick: u64
}

pub fn new(actor_runtime_id: u64, flags: u8, position: Vec<f32>, pitch: f32, yaw: f32, head_yaw: f32, mode: u8, on_ground: bool, riding_actor_runtime_id: u64, teleport_cause: i32, teleport_item: i32, tick: u64) -> MovePlayer {
    MovePlayer { actor_runtime_id, flags, position, pitch, yaw, head_yaw, mode, on_ground, riding_actor_runtime_id, teleport_cause, teleport_item, tick }
}

impl Packet for MovePlayer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMovePlayer.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_byte(self.flags);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        PacketSerializer::put_rotation_byte(&mut stream, self.pitch);
        PacketSerializer::put_rotation_byte(&mut stream, self.yaw);
        PacketSerializer::put_rotation_byte(&mut stream, self.head_yaw);
        stream.put_byte(self.mode);
        stream.put_bool(self.on_ground);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.riding_actor_runtime_id);
        if self.mode == MovePlayer::MODE_TELEPORT {
            stream.put_i32_le(self.teleport_cause);
            stream.put_i32_le(self.teleport_item);
        }
        stream.put_var_u64(self.tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> MovePlayer {
        let mut stream = Stream::new(bytes, 0);

        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let flags = stream.get_byte();
        let position = PacketSerializer::get_vector3(&mut stream);
        let pitch = PacketSerializer::get_rotation_byte(&mut stream);
        let yaw = PacketSerializer::get_rotation_byte(&mut stream);
        let head_yaw = PacketSerializer::get_rotation_byte(&mut stream);
        let mode = stream.get_byte();
        let on_ground = stream.get_bool();
        let riding_actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let (mut teleport_cause, mut teleport_item) = (0, 0);
        if mode == MovePlayer::MODE_TELEPORT {
            teleport_cause = stream.get_i32_le();
            teleport_item = stream.get_i32_le();
        }
        let tick = stream.get_var_u64();

        MovePlayer { actor_runtime_id, flags, position, pitch, yaw, head_yaw, mode, on_ground, riding_actor_runtime_id, teleport_cause, teleport_item, tick }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Flags: {}", self.flags);
        println!("Position: {:?}", self.position);
        println!("Pitch: {}", self.pitch);
        println!("Yaw: {}", self.yaw);
        println!("Head Yaw: {}", self.head_yaw);
        println!("Mode: {}", self.mode);
        println!("On Ground: {}", self.on_ground);
        println!("Riding Actor Runtime ID: {}", self.riding_actor_runtime_id);
        println!("Teleport Cause: {}", self.teleport_cause);
        println!("Teleport Item: {}", self.teleport_item);
        println!("Tick: {}", self.tick);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MovePlayer {
    pub const MODE_NORMAL: u8 = 0;
    pub const MODE_RESET: u8 = 1;
    pub const MODE_TELEPORT: u8 = 2;
    pub const MODE_PITCH: u8 = 3;
}
