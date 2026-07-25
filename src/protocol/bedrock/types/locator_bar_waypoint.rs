use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::world_position::WorldPosition;
use crate::utils::color::Color;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct LocatorBarWaypoint {
    pub update_flag: u32,
    pub visible: Option<bool>,
    pub world_position: Option<WorldPosition>,
    pub texture_path: Option<String>,
    pub icon_size: Option<Vec<f32>>,
    pub color: Option<Color>,
    pub client_position_authority: Option<bool>,
    pub actor_unique_id: Option<i64>
}

impl LocatorBarWaypoint {
    pub fn new(
        update_flag: u32,
        visible: Option<bool>,
        world_position: Option<WorldPosition>,
        texture_path: Option<String>,
        icon_size: Option<Vec<f32>>,
        color: Option<Color>,
        client_position_authority: Option<bool>,
        actor_unique_id: Option<i64>
    ) -> LocatorBarWaypoint {
        LocatorBarWaypoint {
            update_flag,
            visible,
            world_position,
            texture_path,
            icon_size,
            color,
            client_position_authority,
            actor_unique_id,
        }
    }

    pub fn read(stream: &mut Stream) -> LocatorBarWaypoint {
        let update_flag = stream.get_u32_le();
        let visible = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let world_position = PacketSerializer::read_optional(stream, |s| WorldPosition::read(s));
        let texture_path = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let icon_size = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector2(s));
        let color = PacketSerializer::read_optional(stream, |s| Color::from_argb(s.get_u32_le()));
        let client_position_authority = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let actor_unique_id = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_actor_unique_id(s));

        LocatorBarWaypoint {
            update_flag,
            visible,
            world_position,
            texture_path,
            icon_size,
            color,
            client_position_authority,
            actor_unique_id,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_u32_le(self.update_flag);
        PacketSerializer::write_optional(stream, &self.visible, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.world_position, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.texture_path, |s, v| PacketSerializer::put_string(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.icon_size, |s, v| PacketSerializer::put_vector2(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.color, |s, v| s.put_u32_le(v.to_argb()));
        PacketSerializer::write_optional(stream, &self.client_position_authority, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.actor_unique_id, |s, v| PacketSerializer::put_actor_unique_id(s, *v));
    }
}
