use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::biome::chunkgen::biome_definition_chunk_gen_data::BiomeDefinitionChunkGenData;
use crate::utils::color::Color;

#[derive(Debug)]
pub struct BiomeDefinitionData {
    name_index: u16,
    id: u16,
    temperature: f32,
    downfall: f32,
    red_spore_density: f32,
    blue_spore_density: f32,
    ash_density: f32,
    white_ash_density: f32,
    depth: f32,
    scale: f32,
    map_water_color: Color,
    rain: bool,
    tag_indexes: Option<Vec<u16>>,
    chunk_gen_data: Option<BiomeDefinitionChunkGenData>
}

impl BiomeDefinitionData {
    pub fn new(
        name_index: u16,
        id: u16,
        temperature: f32,
        downfall: f32,
        red_spore_density: f32,
        blue_spore_density: f32,
        ash_density: f32,
        white_ash_density: f32,
        depth: f32,
        scale: f32,
        map_water_color: Color,
        rain: bool,
        tag_indexes: Option<Vec<u16>>,
        chunk_gen_data: Option<BiomeDefinitionChunkGenData>
    ) -> BiomeDefinitionData {
        BiomeDefinitionData{
            name_index,
            id,
            temperature,
            downfall,
            red_spore_density,
            blue_spore_density,
            ash_density,
            white_ash_density,
            depth,
            scale,
            map_water_color,
            rain,
            tag_indexes,
            chunk_gen_data
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeDefinitionData {
        let name_index = stream.get_l_short();
        let id = stream.get_l_short();
        let temperature = stream.get_l_float();
        let downfall = stream.get_l_float();
        let red_spore_density = stream.get_l_float();
        let blue_spore_density = stream.get_l_float();
        let ash_density = stream.get_l_float();
        let white_ash_density = stream.get_l_float();
        let depth = stream.get_l_float();
        let scale = stream.get_l_float();
        let map_water_color = Color::from_argb(stream.get_l_int());
        let rain = stream.get_bool();
        let tag_indexes = PacketSerializer::read_optional(stream, |s| {
            let mut sub_tag_indexes = Vec::new();
            let count = s.get_unsigned_var_int();
            for _ in 0..count {
                sub_tag_indexes.push(s.get_l_short());
            }
            sub_tag_indexes
        });
        let chunk_gen_data = PacketSerializer::read_optional(stream, |s| BiomeDefinitionChunkGenData::read(s));

        BiomeDefinitionData{
            name_index,
            id,
            temperature,
            downfall,
            red_spore_density,
            blue_spore_density,
            ash_density,
            white_ash_density,
            depth,
            scale,
            map_water_color,
            rain,
            tag_indexes,
            chunk_gen_data,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_short(self.name_index);
        stream.put_l_short(self.id);
        stream.put_l_float(self.temperature);
        stream.put_l_float(self.downfall);
        stream.put_l_float(self.red_spore_density);
        stream.put_l_float(self.blue_spore_density);
        stream.put_l_float(self.ash_density);
        stream.put_l_float(self.white_ash_density);
        stream.put_l_float(self.depth);
        stream.put_l_float(self.scale);
        stream.put_l_int(self.map_water_color.to_argb());
        stream.put_bool(self.rain);
        PacketSerializer::write_optional(stream, &self.tag_indexes, |s, v| {
            s.put_unsigned_var_int(v.len() as u32);
            for index in v {
                s.put_l_short(*index);
            }
        });
        PacketSerializer::write_optional(stream, &self.chunk_gen_data, |s, v| v.write(s));
    }
}