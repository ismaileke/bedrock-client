use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::world_position::WorldPosition;
use crate::utils::color::Color;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct LocatorBarWaypoint {
    pub update_flag: u32,
    pub visible: Option<bool>,
    pub world_position: Option<WorldPosition>,
    pub texture_id: Option<u32>,
    pub color: Option<Color>,
    pub client_position_authority: Option<bool>,
    pub actor_unique_id: Option<i64>
}

impl LocatorBarWaypoint {
    pub fn new(
        update_flag: u32,
        visible: Option<bool>,
        world_position: Option<WorldPosition>,
        texture_id: Option<u32>,
        color: Option<Color>,
        client_position_authority: Option<bool>,
        actor_unique_id: Option<i64>
    ) -> LocatorBarWaypoint {
        LocatorBarWaypoint {
            update_flag,
            visible,
            world_position,
            texture_id,
            color,
            client_position_authority,
            actor_unique_id,
        }
    }

    pub fn read(stream: &mut Stream) -> LocatorBarWaypoint {
        let update_flag = stream.get_u32_le();
        let visible = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let world_position = PacketSerializer::read_optional(stream, |s| WorldPosition::read(s));
        let texture_id = PacketSerializer::read_optional(stream, |s| s.get_u32_le());
        let color = PacketSerializer::read_optional(stream, |s| Color::from_argb(s.get_u32_le()));
        let client_position_authority = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let actor_unique_id = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_actor_unique_id(s));

        LocatorBarWaypoint {
            update_flag,
            visible,
            world_position,
            texture_id,
            color,
            client_position_authority,
            actor_unique_id,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_u32_le(self.update_flag);
        PacketSerializer::write_optional(stream, &self.visible, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.world_position, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.texture_id, |s, v| s.put_u32_le(*v));
        PacketSerializer::write_optional(stream, &self.color, |s, v| s.put_u32_le(v.to_argb()));
        PacketSerializer::write_optional(stream, &self.client_position_authority, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.actor_unique_id, |s, v| PacketSerializer::put_actor_unique_id(s, *v));
    }
}
