use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::biome::chunkgen::biome_surface_material_data::BiomeSurfaceMaterialData;

#[derive(Debug)]
pub struct BiomeElementData {
    noise_frequency_scale: f32,
    noise_lower_bound: f32,
    noise_upper_bound: f32,
    height_min_type: i32,
    height_min: u16,
    height_max_type: i32,
    height_max: u16,
    surface_material: BiomeSurfaceMaterialData
}

impl BiomeElementData {
    pub fn new(
        noise_frequency_scale: f32,
        noise_lower_bound: f32,
        noise_upper_bound: f32,
        height_min_type: i32,
        height_min: u16,
        height_max_type: i32,
        height_max: u16,
        surface_material: BiomeSurfaceMaterialData
    ) -> Self {
        BiomeElementData{
            noise_frequency_scale,
            noise_lower_bound,
            noise_upper_bound,
            height_min_type,
            height_min,
            height_max_type,
            height_max,
            surface_material
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeElementData {
        let noise_frequency_scale = stream.get_l_float();
        let noise_lower_bound = stream.get_l_float();
        let noise_upper_bound = stream.get_l_float();
        let height_min_type = stream.get_var_int();
        let height_min = stream.get_l_short();
        let height_max_type = stream.get_var_int();
        let height_max = stream.get_l_short();
        let surface_material = BiomeSurfaceMaterialData::read(stream);

        BiomeElementData::new(noise_frequency_scale, noise_lower_bound, noise_upper_bound, height_min_type, height_min, height_max_type, height_max, surface_material)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_float(self.noise_frequency_scale);
        stream.put_l_float(self.noise_lower_bound);
        stream.put_l_float(self.noise_upper_bound);
        stream.put_var_int(self.height_min_type);
        stream.put_l_short(self.height_min);
        stream.put_var_int(self.height_max_type);
        stream.put_l_short(self.height_max);
        self.surface_material.write(stream);
    }
}