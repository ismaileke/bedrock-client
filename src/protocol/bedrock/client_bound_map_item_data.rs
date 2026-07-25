use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;
//use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundMapItemData {
    /*pub map_id: i64,
    pub map_type: u32,
    pub dimension_id: u8,
    pub is_locked: bool,
    pub origin: Vec<i32>*/
}

impl Packet for ClientBoundMapItemData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundMapItemData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        // TODO

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(_stream: &mut Stream) -> ClientBoundMapItemData {
        /*let map_id = PacketSerializer::get_actor_unique_id(stream);
        let map_type = stream.get_var_u32();
        let dimension_id = stream.get_byte();
        let is_locked = stream.get_bool();
        let origin = PacketSerializer::get_block_pos(stream);

        if (map_type & Self::BITFLAG_MAP_CREATION) != 0 {
            let count = stream.get_var_u32() as usize;
            let mut parent_map_ids = Vec::with_capacity(count);
            for _ in 0..count {
                parent_map_ids.push(PacketSerializer::get_actor_unique_id(stream));
            }
        }

        if (map_type & (Self::BITFLAG_MAP_CREATION | Self::BITFLAG_DECORATION_UPDATE | Self::BITFLAG_TEXTURE_UPDATE)) != 0 { //Decoration bitflag or colour bitflag
            let scale = stream.get_byte();
        }

        if (map_type & Self::BITFLAG_DECORATION_UPDATE) != 0 {
            let count = stream.get_var_u32() as usize;
            let mut tracked_entities = Vec::with_capacity(count);
            for _ in 0..count {

                let object = MapTrackedObject {
                    object_type: stream.get_u32_le(),

                };

            }
        }*/

        ClientBoundMapItemData {}
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}

impl ClientBoundMapItemData {
    pub const BITFLAG_TEXTURE_UPDATE: u32 = 0x02;
    pub const BITFLAG_DECORATION_UPDATE: u32 = 0x04;
    pub const BITFLAG_MAP_CREATION: u32 = 0x08;
}
