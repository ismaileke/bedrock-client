use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::biome::chunkgen::biome_definition_chunk_gen_data::BiomeDefinitionChunkGenData;
use crate::utils::color::Color;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeDefinitionData {
    name_index: u16,
    id: u16,
    temperature: f32,
    downfall: f32,
    foliage_snow: f32,
    depth: f32,
    scale: f32,
    map_water_color: Color,
    rain: bool,
    tag_indexes: Option<Vec<u16>>,
    chunk_gen_data: Option<BiomeDefinitionChunkGenData>,
}

impl BiomeDefinitionData {
    pub fn new(
        name_index: u16,
        id: u16,
        temperature: f32,
        downfall: f32,
        foliage_snow: f32,
        depth: f32,
        scale: f32,
        map_water_color: Color,
        rain: bool,
        tag_indexes: Option<Vec<u16>>,
        chunk_gen_data: Option<BiomeDefinitionChunkGenData>,
    ) -> BiomeDefinitionData {
        BiomeDefinitionData {
            name_index,
            id,
            temperature,
            downfall,
            foliage_snow,
            depth,
            scale,
            map_water_color,
            rain,
            tag_indexes,
            chunk_gen_data,
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeDefinitionData {
        let name_index = stream.get_u16_le();
        let id = stream.get_u16_le();
        let temperature = stream.get_f32_le();
        let downfall = stream.get_f32_le();
        let foliage_snow = stream.get_f32_le();
        let depth = stream.get_f32_le();
        let scale = stream.get_f32_le();
        let map_water_color = Color::from_argb(stream.get_u32_le());
        let rain = stream.get_bool();
        let tag_indexes = PacketSerializer::read_optional(stream, |s| {
            let mut sub_tag_indexes = Vec::new();
            let count = s.get_var_u32();
            for _ in 0..count {
                sub_tag_indexes.push(s.get_u16_le());
            }
            sub_tag_indexes
        });
        let chunk_gen_data =
            PacketSerializer::read_optional(stream, |s| BiomeDefinitionChunkGenData::read(s));

        BiomeDefinitionData {
            name_index,
            id,
            temperature,
            downfall,
            foliage_snow,
            depth,
            scale,
            map_water_color,
            rain,
            tag_indexes,
            chunk_gen_data,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_u16_le(self.name_index);
        stream.put_u16_le(self.id);
        stream.put_f32_le(self.temperature);
        stream.put_f32_le(self.downfall);
        stream.put_f32_le(self.foliage_snow);
        stream.put_f32_le(self.depth);
        stream.put_f32_le(self.scale);
        stream.put_u32_le(self.map_water_color.to_argb());
        stream.put_bool(self.rain);
        PacketSerializer::write_optional(stream, &self.tag_indexes, |s, v| {
            s.put_var_u32(v.len() as u32);
            for index in v {
                s.put_u16_le(*index);
            }
        });
        PacketSerializer::write_optional(stream, &self.chunk_gen_data, |s, v| v.write(s));
    }
}
