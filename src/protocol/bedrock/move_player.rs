use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
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
    pub tick: u64,
}

impl Packet for MovePlayer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMovePlayer.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        stream.put_u8(self.flags);
        PacketSerializer::put_vector3(stream, self.position.clone());
        PacketSerializer::put_rotation_byte(stream, self.pitch);
        PacketSerializer::put_rotation_byte(stream, self.yaw);
        PacketSerializer::put_rotation_byte(stream, self.head_yaw);
        stream.put_u8(self.mode);
        stream.put_bool(self.on_ground);
        PacketSerializer::put_actor_runtime_id(stream, self.riding_actor_runtime_id);
        if self.mode == MovePlayer::MODE_TELEPORT {
            stream.put_i32_le(self.teleport_cause);
            stream.put_i32_le(self.teleport_item);
        }
        stream.put_var_u64(self.tick);
    }

    fn decode(stream: &mut Reader) -> MovePlayer {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let flags = stream.get_u8();
        let position = PacketSerializer::get_vector3(stream);
        let pitch = PacketSerializer::get_rotation_byte(stream);
        let yaw = PacketSerializer::get_rotation_byte(stream);
        let head_yaw = PacketSerializer::get_rotation_byte(stream);
        let mode = stream.get_u8();
        let on_ground = stream.get_bool();
        let riding_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let (mut teleport_cause, mut teleport_item) = (0, 0);
        if mode == MovePlayer::MODE_TELEPORT {
            teleport_cause = stream.get_i32_le();
            teleport_item = stream.get_i32_le();
        }
        let tick = stream.get_var_u64();

        MovePlayer {
            actor_runtime_id,
            flags,
            position,
            pitch,
            yaw,
            head_yaw,
            mode,
            on_ground,
            riding_actor_runtime_id,
            teleport_cause,
            teleport_item,
            tick,
        }
    }
}

impl MovePlayer {
    pub const MODE_NORMAL: u8 = 0;
    pub const MODE_RESET: u8 = 1;
    pub const MODE_TELEPORT: u8 = 2;
    pub const MODE_PITCH: u8 = 3;
}
