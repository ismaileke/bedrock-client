use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::map_decoration::MapDecoration;
use crate::protocol::bedrock::types::map_image::MapImage;
use crate::protocol::bedrock::types::map_tracked_object::MapTrackedObject;
use crate::utils::color::Color;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundMapItemData {
    pub map_id: i64,
    pub map_type: u32,
    pub dimension_id: u8,
    pub is_locked: bool,
    pub origin: Vec<i32>,
    pub parent_map_ids: Vec<i64>,
    pub scale: u8,
    pub tracked_entities: Vec<MapTrackedObject>,
    pub decorations: Vec<MapDecoration>,
    pub x_offset: i32,
    pub y_offset: i32,
    pub colors: Option<MapImage>
}

impl Packet for ClientBoundMapItemData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundMapItemData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.map_id);
        let mut map_type = 0;
        let parent_map_ids_count = self.parent_map_ids.len() as u32;
        if parent_map_ids_count > 0 {
            map_type |= Self::BITFLAG_MAP_CREATION;
        }
        let decoration_count  = self.decorations.len() as u32;
        if decoration_count > 0 {
            map_type |= Self::BITFLAG_DECORATION_UPDATE;
        }
        if self.colors.is_some() {
            map_type |= Self::BITFLAG_TEXTURE_UPDATE;
        }
        stream.put_var_u32(map_type);
        stream.put_u8(self.dimension_id);
        stream.put_bool(self.is_locked);
        PacketSerializer::put_block_pos(stream, self.origin.clone());
        if (map_type & Self::BITFLAG_MAP_CREATION) != 0 {
            stream.put_var_u32(parent_map_ids_count);
            for parent_map_id in &self.parent_map_ids {
                PacketSerializer::put_actor_unique_id(stream, *parent_map_id);
            }
        }
        if (map_type & (Self::BITFLAG_MAP_CREATION | Self::BITFLAG_DECORATION_UPDATE | Self::BITFLAG_TEXTURE_UPDATE)) != 0 {
            stream.put_u8(self.scale);
        }
        if (map_type & Self::BITFLAG_DECORATION_UPDATE) != 0 {
            stream.put_var_u32(self.tracked_entities.len() as u32);
            for tracked_entity in &self.tracked_entities {
                stream.put_var_u32(tracked_entity.object_type);
                if tracked_entity.object_type == MapTrackedObject::TYPE_BLOCK {
                    PacketSerializer::put_block_pos(stream, tracked_entity.block_position.clone().unwrap());
                } else if tracked_entity.object_type == MapTrackedObject::TYPE_ENTITY {
                    PacketSerializer::put_actor_unique_id(stream, tracked_entity.actor_unique_id.clone().unwrap());
                } else {
                    panic!("Unknown map object type {}", tracked_entity.object_type);
                }
            }
            stream.put_var_u32(decoration_count);
            for decoration in &self.decorations {
                stream.put_u8(decoration.icon);
                stream.put_u8(decoration.rotation);
                stream.put_u8(decoration.x_offset);
                stream.put_u8(decoration.y_offset);
                PacketSerializer::put_string(stream, decoration.label.clone());
                stream.put_var_u32(Self::flip_int_endianness(decoration.color.to_rgba()));
            }
        }
        if let Some(colors) = &mut self.colors {
            stream.put_var_i32(colors.width);
            stream.put_var_i32(colors.height);
            stream.put_var_i32(self.x_offset);
            stream.put_var_i32(self.y_offset);
            stream.put_var_u32((colors.width * colors.height) as u32); // I'm not sure if this is correct
            colors.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> ClientBoundMapItemData {
        let map_id = PacketSerializer::get_actor_unique_id(stream);
        let map_type = stream.get_var_u32();
        let dimension_id = stream.get_u8();
        let is_locked = stream.get_bool();
        let origin = PacketSerializer::get_block_pos(stream);

        let mut parent_map_ids = Vec::new();
        if (map_type & Self::BITFLAG_MAP_CREATION) != 0 {
            let count = stream.get_var_u32() as usize;
            for _ in 0..count {
                parent_map_ids.push(PacketSerializer::get_actor_unique_id(stream));
            }
        }

        let mut scale = 0; // I don't think so
        if (map_type & (Self::BITFLAG_MAP_CREATION | Self::BITFLAG_DECORATION_UPDATE | Self::BITFLAG_TEXTURE_UPDATE)) != 0 { //Decoration bitflag or color bitflag
            scale = stream.get_u8();
        }

        let mut tracked_entities = Vec::new();
        let mut decorations = Vec::new();
        if (map_type & Self::BITFLAG_DECORATION_UPDATE) != 0 {
            let mut count = stream.get_var_u32() as usize;
            for _ in 0..count {
                let object_type = stream.get_var_u32();
                let mut block_position = None;
                let mut actor_unique_id = None;
                if object_type == MapTrackedObject::TYPE_BLOCK {
                    block_position = Some(PacketSerializer::get_block_pos(stream));
                } else if object_type == MapTrackedObject::TYPE_ENTITY {
                    actor_unique_id = Some(PacketSerializer::get_actor_unique_id(stream));
                } else {
                    panic!("Unknown map object type {}", object_type);
                }
                tracked_entities.push(MapTrackedObject { object_type, actor_unique_id, block_position });

            }

            count = stream.get_var_u32() as usize;
            for _ in 0..count {
                let icon = stream.get_u8();
                let rotation = stream.get_u8();
                let x_offset = stream.get_u8();
                let y_offset = stream.get_u8();
                let label = PacketSerializer::get_string(stream);
                let color = Color::from_rgba(Self::flip_int_endianness(stream.get_var_u32()));
                decorations.push(MapDecoration { icon, rotation, x_offset, y_offset, label, color });
            }
        }

        let mut x_offset = 0;
        let mut y_offset = 0;
        let mut colors = None;
        if (map_type & Self::BITFLAG_TEXTURE_UPDATE) != 0 {
            let width = stream.get_var_i32();
            let height = stream.get_var_i32();
            x_offset = stream.get_var_i32();
            y_offset = stream.get_var_i32();

            let count = stream.get_var_u32();
            if count != (width * height) as u32 { // I'm not sure if this is correct
                panic!("Expected colour count of {} (height {} * width {}), got {}", count, height, width, count);
            }

            colors = Some(MapImage::read(stream, height, width));
        }

        ClientBoundMapItemData {
            map_id,
            map_type,
            dimension_id,
            is_locked,
            origin,
            parent_map_ids,
            scale,
            tracked_entities,
            decorations,
            x_offset,
            y_offset,
            colors,
        }
    }
}

impl ClientBoundMapItemData {
    pub const BITFLAG_TEXTURE_UPDATE: u32 = 0x02;
    pub const BITFLAG_DECORATION_UPDATE: u32 = 0x04;
    pub const BITFLAG_MAP_CREATION: u32 = 0x08;

    pub fn flip_int_endianness(value: u32) -> u32 { // just for now, until we have a proper endianess function
        let mut stream = Writer::new();
        stream.put_u32_be(value);
        let mut stream2 = Reader::new(stream.as_slice());
        stream2.get_u32_le() // dirty way to flip the endianness
    }
}
