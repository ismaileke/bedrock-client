use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MoveActorAbsolute {
    pub actor_runtime_id: u64,
    pub flags: u8,
    pub position: Vec<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub head_yaw: f32, // always zero for non-mobs
}

impl Packet for MoveActorAbsolute {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMoveActorAbsolute.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        stream.put_u8(self.flags);
        PacketSerializer::put_vector3(stream, &self.position);
        PacketSerializer::put_rotation_byte(stream, self.pitch);
        PacketSerializer::put_rotation_byte(stream, self.yaw);
        PacketSerializer::put_rotation_byte(stream, self.head_yaw);
    }

    fn decode(stream: &mut Reader) -> MoveActorAbsolute {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let flags = stream.get_u8();
        let position = PacketSerializer::get_vector3(stream);
        let pitch = PacketSerializer::get_rotation_byte(stream);
        let yaw = PacketSerializer::get_rotation_byte(stream);
        let head_yaw = PacketSerializer::get_rotation_byte(stream);

        MoveActorAbsolute { actor_runtime_id, flags, position, pitch, yaw, head_yaw }
    }
}

impl MoveActorAbsolute {
    pub const FLAG_GROUND: u8 = 0x01;
    pub const FLAG_TELEPORT: u8 = 0x02;
    pub const FLAG_FORCE_MOVE_LOCAL_ENTITY: u8 = 0x04;
    pub const FLAG_FORCE_COMPLETION: u8 = 0x08;
}
