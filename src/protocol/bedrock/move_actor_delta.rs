use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MoveActorDelta {
    pub actor_runtime_id: u64,
    pub pos_x: Option<f32>,
    pub pos_y: Option<f32>,
    pub pos_z: Option<f32>,
    pub rotation_x: Option<f32>,
    pub rotation_y: Option<f32>,
    pub rotation_y_head: Option<f32>,
    pub on_ground: bool,
    pub force_move: bool,
    pub force_move_local_entity: bool,
    pub force_completion: bool
}

impl Packet for MoveActorDelta {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMoveActorDelta.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        PacketSerializer::write_optional(stream, &self.pos_x, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.pos_y, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.pos_z, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.rotation_x, |s, v| PacketSerializer::put_rotation_byte(s, *v));
        PacketSerializer::write_optional(stream, &self.rotation_y, |s, v| PacketSerializer::put_rotation_byte(s, *v));
        PacketSerializer::write_optional(stream, &self.rotation_y_head, |s, v| PacketSerializer::put_rotation_byte(s, *v));
        stream.put_bool(self.on_ground);
        stream.put_bool(self.force_move);
        stream.put_bool(self.force_move_local_entity);
        stream.put_bool(self.force_completion);
    }

    fn decode(stream: &mut Reader) -> MoveActorDelta {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let pos_x = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let pos_y = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let pos_z = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let rotation_x = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_rotation_byte(s));
        let rotation_y = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_rotation_byte(s));
        let rotation_y_head = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_rotation_byte(s));
        let on_ground = stream.get_bool();
        let force_move = stream.get_bool();
        let force_move_local_entity = stream.get_bool();
        let force_completion = stream.get_bool();

        MoveActorDelta {
            actor_runtime_id,
            pos_x,
            pos_y,
            pos_z,
            rotation_x,
            rotation_y,
            rotation_y_head,
            on_ground,
            force_move,
            force_move_local_entity,
            force_completion
        }
    }
}
