use crate::protocol::bedrock::types::biome::chunkgen::biome_surface_material_data::BiomeSurfaceMaterialData;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeElementData {
    pub noise_frequency_scale: f32,
    pub noise_lower_bound: f32,
    pub noise_upper_bound: f32,
    pub height_min_type: i32,
    pub height_min: i16,
    pub height_max_type: i32,
    pub height_max: i16,
    pub surface_material: BiomeSurfaceMaterialData,
}

impl BiomeElementData {
    pub fn new(
        noise_frequency_scale: f32,
        noise_lower_bound: f32,
        noise_upper_bound: f32,
        height_min_type: i32,
        height_min: i16,
        height_max_type: i32,
        height_max: i16,
        surface_material: BiomeSurfaceMaterialData,
    ) -> Self {
        BiomeElementData {
            noise_frequency_scale,
            noise_lower_bound,
            noise_upper_bound,
            height_min_type,
            height_min,
            height_max_type,
            height_max,
            surface_material,
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeElementData {
        let noise_frequency_scale = stream.get_f32_le();
        let noise_lower_bound = stream.get_f32_le();
        let noise_upper_bound = stream.get_f32_le();
        let height_min_type = stream.get_var_i32();
        let height_min = stream.get_i16_le();
        let height_max_type = stream.get_var_i32();
        let height_max = stream.get_i16_le();
        let surface_material = BiomeSurfaceMaterialData::read(stream);

        BiomeElementData::new(
            noise_frequency_scale,
            noise_lower_bound,
            noise_upper_bound,
            height_min_type,
            height_min,
            height_max_type,
            height_max,
            surface_material,
        )
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.noise_frequency_scale);
        stream.put_f32_le(self.noise_lower_bound);
        stream.put_f32_le(self.noise_upper_bound);
        stream.put_var_i32(self.height_min_type);
        stream.put_i16_le(self.height_min);
        stream.put_var_i32(self.height_max_type);
        stream.put_i16_le(self.height_max);
        self.surface_material.write(stream);
    }
}
