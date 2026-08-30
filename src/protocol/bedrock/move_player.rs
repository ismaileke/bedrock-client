use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MovePlayer {
    pub actor_runtime_id: u64,
    pub position: Vec<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub head_yaw: f32,
    pub mode: u8,
    pub on_ground: bool,
    pub riding_actor_runtime_id: u64,
    pub teleport_cause: Option<i32>,
    pub teleport_item: Option<i32>,
    pub tick: u64,
}

impl Packet for MovePlayer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMovePlayer.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        PacketSerializer::put_vector3(stream, &self.position);
        stream.put_f32_le(self.pitch);
        stream.put_f32_le(self.yaw);
        stream.put_f32_le(self.head_yaw);
        stream.put_u8(self.mode);
        stream.put_bool(self.on_ground);
        PacketSerializer::put_actor_runtime_id(stream, self.riding_actor_runtime_id);
        let has_teleport = self.teleport_cause.is_some() && self.teleport_item.is_some();
        stream.put_bool(has_teleport);
        if has_teleport {
            stream.put_i32_le(self.teleport_cause.unwrap());
            stream.put_i32_le(self.teleport_item.unwrap());
        }
        stream.put_var_u64(self.tick);
    }

    fn decode(stream: &mut Reader) -> MovePlayer {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let position = PacketSerializer::get_vector3(stream);
        let pitch = stream.get_f32_le();
        let yaw = stream.get_f32_le();
        let head_yaw = stream.get_f32_le();
        let mode = stream.get_u8();
        let on_ground = stream.get_bool();
        let riding_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let (mut teleport_cause, mut teleport_item) = (None, None);
        if stream.get_bool() {
            teleport_cause = Some(stream.get_i32_le());
            teleport_item = Some(stream.get_i32_le());
        }
        let tick = stream.get_var_u64();

        MovePlayer {
            actor_runtime_id,
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
