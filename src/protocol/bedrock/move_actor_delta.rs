use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MoveActorDelta {
    pub actor_runtime_id: u64,
    pub flags: u16,
    pub x_pos: f32,
    pub y_pos: f32,
    pub z_pos: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub head_yaw: f32,
}

impl Packet for MoveActorDelta {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMoveActorDelta.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        stream.put_u16_le(self.flags);

        MoveActorDelta::maybe_write_coord(self.flags, Self::FLAG_HAS_X, self.x_pos, stream);
        MoveActorDelta::maybe_write_coord(self.flags, Self::FLAG_HAS_Y, self.y_pos, stream);
        MoveActorDelta::maybe_write_coord(self.flags, Self::FLAG_HAS_Z, self.z_pos, stream);
        MoveActorDelta::maybe_write_rotation(self.flags, Self::FLAG_HAS_PITCH, self.pitch, stream);
        MoveActorDelta::maybe_write_rotation(self.flags, Self::FLAG_HAS_YAW, self.yaw, stream);
        MoveActorDelta::maybe_write_rotation(self.flags, Self::FLAG_HAS_HEAD_YAW, self.head_yaw, stream);
    }

    fn decode(stream: &mut Reader) -> MoveActorDelta {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let flags = stream.get_u16_le();
        let x_pos = MoveActorDelta::maybe_read_coord(flags, Self::FLAG_HAS_X, stream);
        let y_pos = MoveActorDelta::maybe_read_coord(flags, Self::FLAG_HAS_Y, stream);
        let z_pos = MoveActorDelta::maybe_read_coord(flags, Self::FLAG_HAS_Z, stream);
        let pitch = MoveActorDelta::maybe_read_rotation(flags, Self::FLAG_HAS_PITCH, stream);
        let yaw = MoveActorDelta::maybe_read_rotation(flags, Self::FLAG_HAS_YAW, stream);
        let head_yaw = MoveActorDelta::maybe_read_rotation(flags, Self::FLAG_HAS_HEAD_YAW, stream);

        MoveActorDelta {
            actor_runtime_id,
            flags,
            x_pos,
            y_pos,
            z_pos,
            pitch,
            yaw,
            head_yaw,
        }
    }
}

impl MoveActorDelta {
    pub const FLAG_HAS_X: u16 = 0x01;
    pub const FLAG_HAS_Y: u16 = 0x02;
    pub const FLAG_HAS_Z: u16 = 0x04;
    pub const FLAG_HAS_PITCH: u16 = 0x08;
    pub const FLAG_HAS_YAW: u16 = 0x10;
    pub const FLAG_HAS_HEAD_YAW: u16 = 0x20;
    pub const FLAG_GROUND: u16 = 0x40;
    pub const FLAG_TELEPORT: u16 = 0x80;
    pub const FLAG_FORCE_MOVE_LOCAL_ENTITY: u16 = 0x100;

    pub fn maybe_read_coord(flags: u16, flag: u16, stream: &mut Reader) -> f32 {
        if flags & flag != 0 {
            stream.get_f32_le()
        } else {
            0.0
        }
    }

    pub fn maybe_write_coord(flags: u16, flag: u16, float_val: f32, stream: &mut Writer) {
        if flags & flag != 0 {
            stream.put_f32_le(float_val);
        }
    }

    pub fn maybe_read_rotation(flags: u16, flag: u16, stream: &mut Reader) -> f32 {
        if flags & flag != 0 {
            PacketSerializer::get_rotation_byte(stream)
        } else {
            0.0
        }
    }

    pub fn maybe_write_rotation(flags: u16, flag: u16, float_val: f32, stream: &mut Writer) {
        if flags & flag != 0 {
            PacketSerializer::put_rotation_byte(stream, float_val);
        }
    }
}
