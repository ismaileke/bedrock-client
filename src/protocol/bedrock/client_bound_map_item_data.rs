use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::map_decoration::MapDecoration;
use crate::protocol::bedrock::types::map_tracked_object::MapTrackedObject;
use crate::utils::color::Color;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundMapItemData {
    pub map_id: i64,
    pub dimension_id: u8,
    pub is_locked: bool,
    pub origin: Vec<i32>,
    pub tracked_entities: Option<Vec<i64>>,
    pub scale: Option<u8>,
    pub tracked_objects: Option<Vec<MapTrackedObject>>,
    pub decorations: Option<Vec<MapDecoration>>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub x_offset: Option<i32>,
    pub y_offset: Option<i32>,
    pub colors: Option<Vec<i32>>
}

impl Packet for ClientBoundMapItemData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundMapItemData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.map_id);
        stream.put_u8(self.dimension_id);
        stream.put_bool(self.is_locked);
        PacketSerializer::put_block_pos(stream, &self.origin);

        PacketSerializer::write_optional(stream, &self.tracked_entities, |s, v| {
            s.put_var_u32(v.len() as u32);
            v.iter().for_each(|e| PacketSerializer::put_actor_unique_id(s, *e))
        });

        PacketSerializer::write_optional(stream, &self.scale, |s, v| s.put_u8(*v));

        PacketSerializer::write_optional(stream, &self.tracked_objects, |s, v| {
            s.put_var_u32(v.len() as u32);
            v.iter().for_each(|e| {
                s.put_i32_le(e.object_type);
                match e.object_type {
                    MapTrackedObject::TYPE_ENTITY => {
                        s.put_bool(true);
                        PacketSerializer::put_actor_unique_id(s, e.actor_unique_id.unwrap());
                        s.put_bool(false);
                    },
                    MapTrackedObject::TYPE_BLOCK_ENTITY => {
                        s.put_bool(false);
                        s.put_bool(true);
                        if let Some(block_position) = &e.block_position {
                            PacketSerializer::put_block_pos(s, block_position);
                        } else { panic!("Block position is None"); }
                    },
                    _ => { panic!("Invalid object type"); }
                }
            });
        });

        PacketSerializer::write_optional(stream, &self.decorations, |s, v| {
            s.put_var_u32(v.len() as u32);
            v.iter().for_each(|e| {
                s.put_u8(e.icon);
                s.put_u8(e.rotation);
                s.put_u8(e.x_offset);
                s.put_u8(e.y_offset);
                PacketSerializer::put_string(s, &e.label);
                s.put_u32_le(Self::flip_int_endianness(e.color.to_rgba()));
            });
        });

        PacketSerializer::write_optional(stream, &self.width, |s, v| s.put_var_i32(*v));
        PacketSerializer::write_optional(stream, &self.height, |s, v| s.put_var_i32(*v));
        PacketSerializer::write_optional(stream, &self.x_offset, |s, v| s.put_var_i32(*v));
        PacketSerializer::write_optional(stream, &self.y_offset, |s, v| s.put_var_i32(*v));

        PacketSerializer::write_optional(stream, &self.colors, |s, v| {
            s.put_var_u32(v.len() as u32);
            v.iter().for_each(|e| s.put_i32_le(*e));
        });
    }

    fn decode(stream: &mut Reader) -> ClientBoundMapItemData {
        let map_id = PacketSerializer::get_actor_unique_id(stream);
        let dimension_id = stream.get_u8();
        let is_locked = stream.get_bool();
        let origin = PacketSerializer::get_block_pos(stream);

        let mut tracked_entities: Option<Vec<i64>> = None;
        if stream.get_bool() {
            let mut tracked_entity_ids = Vec::new();
            let len = stream.get_var_u32() as usize;
            for _ in 0..len {
                tracked_entity_ids.push(PacketSerializer::get_actor_unique_id(stream));
            }
            tracked_entities = Some(tracked_entity_ids);
        }

        let mut scale: Option<u8> = None;
        if stream.get_bool() {
            scale = Some(stream.get_u8());
        }

        let mut tracked_objects: Option<Vec<MapTrackedObject>> = None;
        if stream.get_bool() {
            let mut tracked_object_list = Vec::new();
            let len = stream.get_var_u32() as usize;
            for _ in 0..len {
                let object_type = stream.get_i32_le();
                let mut actor_unique_id = None;
                let mut block_position = None;
                if stream.get_bool() {
                    actor_unique_id = Some(PacketSerializer::get_actor_unique_id(stream));
                }
                if stream.get_bool() {
                    block_position = Some(PacketSerializer::get_block_pos(stream));
                }
                tracked_object_list.push(MapTrackedObject { object_type, actor_unique_id, block_position });
            }
            tracked_objects = Some(tracked_object_list);
        }

        let mut decorations: Option<Vec<MapDecoration>> = None;
        if stream.get_bool() {
            let mut decoration_list = Vec::new();
            let len = stream.get_var_u32() as usize;
            for _ in 0..len {
                let icon = stream.get_u8();
                let rotation = stream.get_u8();
                let x_offset = stream.get_u8();
                let y_offset = stream.get_u8();
                let label = PacketSerializer::get_string(stream);
                let color = Color::from_rgba(Self::flip_int_endianness(stream.get_u32_le()));
                decoration_list.push(MapDecoration { icon, rotation, x_offset, y_offset, label, color });
            }
            decorations = Some(decoration_list);
        }

        let width = PacketSerializer::read_optional(stream, |stream| stream.get_var_i32());
        let height = PacketSerializer::read_optional(stream, |stream| stream.get_var_i32());
        let x_offset = PacketSerializer::read_optional(stream, |stream| stream.get_var_i32());
        let y_offset = PacketSerializer::read_optional(stream, |stream| stream.get_var_i32());

        let mut colors: Option<Vec<i32>> = None;
        if stream.get_bool() {
            let mut colors_buffer = Vec::new();
            let len = stream.get_var_u32();
            for _ in 0..len {
                colors_buffer.push(stream.get_i32_le());
            }
            colors = Some(colors_buffer);
        }

        ClientBoundMapItemData {
            map_id,
            dimension_id,
            is_locked,
            origin,
            tracked_entities,
            scale,
            tracked_objects,
            decorations,
            width,
            height,
            x_offset,
            y_offset,
            colors
        }
    }
}

impl ClientBoundMapItemData {
    pub fn flip_int_endianness(value: u32) -> u32 { // just for now, until we have a proper endianess function
        let mut stream = Writer::new();
        stream.put_u32_be(value);
        let mut stream2 = Reader::new(stream.as_slice());
        stream2.get_u32_le() // dirty way to flip the endianness
    }
}
