use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::sub_chunk_height_map_info::SubChunkHeightMapInfo;
use crate::protocol::bedrock::types::sub_chunk_height_map_type::SubChunkHeightMapType;
use crate::protocol::bedrock::types::sub_chunk_position_offset::SubChunkPositionOffset;
use crate::protocol::bedrock::types::sub_chunk_request_result::SubChunkRequestResult;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SubChunkEntryCommon {
    offset: SubChunkPositionOffset,
    request_result: u8,
    terrain_data: String,
    height_map: Option<SubChunkHeightMapInfo>,
    render_height_map: Option<SubChunkHeightMapInfo>,
}

impl SubChunkEntryCommon {
    pub fn new(
        offset: SubChunkPositionOffset,
        request_result: u8,
        terrain_data: String,
        height_map: Option<SubChunkHeightMapInfo>,
        render_height_map: Option<SubChunkHeightMapInfo>,
    ) -> SubChunkEntryCommon {
        SubChunkEntryCommon {
            offset,
            request_result,
            terrain_data,
            height_map,
            render_height_map,
        }
    }

    pub fn read(stream: &mut Stream, cache_enabled: bool) -> SubChunkEntryCommon {
        let offset = SubChunkPositionOffset::read(stream);
        let request_result = stream.get_byte();
        let terrain_data =
            if !cache_enabled || request_result != SubChunkRequestResult::SUCCESS_ALL_AIR {
                PacketSerializer::get_string(stream)
            } else {
                String::new()
            };

        let height_map_data_type = stream.get_byte();
        let height_map = match height_map_data_type {
            SubChunkHeightMapType::NO_DATA => None,
            SubChunkHeightMapType::DATA => Some(SubChunkHeightMapInfo::read(stream)),
            SubChunkHeightMapType::ALL_TOO_HIGH => Some(SubChunkHeightMapInfo::all_too_high()),
            SubChunkHeightMapType::ALL_TOO_LOW => Some(SubChunkHeightMapInfo::all_too_low()),
            _ => panic!("Unknown heightmap data type {}", height_map_data_type),
        };

        let render_height_map_data_type = stream.get_byte();
        let render_height_map = match render_height_map_data_type {
            SubChunkHeightMapType::NO_DATA => None,
            SubChunkHeightMapType::DATA => Some(SubChunkHeightMapInfo::read(stream)),
            SubChunkHeightMapType::ALL_TOO_HIGH => Some(SubChunkHeightMapInfo::all_too_high()),
            SubChunkHeightMapType::ALL_TOO_LOW => Some(SubChunkHeightMapInfo::all_too_low()),
            SubChunkHeightMapType::ALL_COPIED => height_map.clone(),
            _ => panic!(
                "Unknown render heightmap data type {}",
                height_map_data_type
            ),
        };

        SubChunkEntryCommon {
            offset,
            request_result,
            terrain_data,
            height_map,
            render_height_map,
        }
    }

    pub fn write(&self, stream: &mut Stream, cache_enabled: bool) {
        self.offset.write(stream);
        stream.put_byte(self.request_result);

        if !cache_enabled || self.request_result != SubChunkRequestResult::SUCCESS_ALL_AIR {
            PacketSerializer::put_string(stream, self.terrain_data.clone());
        }

        if let Some(height_map) = &self.height_map {
            if height_map.is_all_too_low() {
                stream.put_byte(SubChunkHeightMapType::ALL_TOO_LOW);
            } else if height_map.is_all_too_high() {
                stream.put_byte(SubChunkHeightMapType::ALL_TOO_HIGH);
            } else {
                stream.put_byte(SubChunkHeightMapType::DATA);
                height_map.write(stream);
            }
        } else {
            stream.put_byte(SubChunkHeightMapType::NO_DATA);
        }

        if let Some(render_height_map) = &self.render_height_map {
            if render_height_map.is_all_too_low() {
                stream.put_byte(SubChunkHeightMapType::ALL_TOO_LOW);
            } else if render_height_map.is_all_too_high() {
                stream.put_byte(SubChunkHeightMapType::ALL_TOO_HIGH);
            } else {
                stream.put_byte(SubChunkHeightMapType::DATA);
                render_height_map.write(stream);
            }
        } else {
            stream.put_byte(SubChunkHeightMapType::ALL_COPIED);
        }
    }
}
