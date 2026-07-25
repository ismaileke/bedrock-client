use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::biome::chunkgen::biome_capped_surface_data::BiomeCappedSurfaceData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_mesa_surface_data::BiomeMesaSurfaceData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_surface_material_data::BiomeSurfaceMaterialData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_noise_gradient_surface_data::BiomeNoiseGradientSurfaceData;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeSurfaceBuilderData {
    pub surface_material: Option<BiomeSurfaceMaterialData>,
    pub default_overworld_surface: bool,
    pub swamp_surface: bool,
    pub frozen_ocean_surface: bool,
    pub the_end_surface: bool,
    pub mesa_surface: Option<BiomeMesaSurfaceData>,
    pub capped_surface: Option<BiomeCappedSurfaceData>,
    pub noise_gradient_surface: Option<BiomeNoiseGradientSurfaceData>,
}
impl BiomeSurfaceBuilderData {
    pub fn new(
        surface_material: Option<BiomeSurfaceMaterialData>,
        default_overworld_surface: bool,
        swamp_surface: bool,
        frozen_ocean_surface: bool,
        the_end_surface: bool,
        mesa_surface: Option<BiomeMesaSurfaceData>,
        capped_surface: Option<BiomeCappedSurfaceData>,
        noise_gradient_surface: Option<BiomeNoiseGradientSurfaceData>
    ) -> Self {
        BiomeSurfaceBuilderData { surface_material, default_overworld_surface, swamp_surface, frozen_ocean_surface, the_end_surface, mesa_surface, capped_surface, noise_gradient_surface }
    }

    pub fn read(stream: &mut Stream) -> BiomeSurfaceBuilderData {
        let surface_material = PacketSerializer::read_optional(stream, |s| BiomeSurfaceMaterialData::read(s));
        let default_overworld_surface = stream.get_bool();
        let swamp_surface = stream.get_bool();
        let frozen_ocean_surface = stream.get_bool();
        let the_end_surface = stream.get_bool();
        let mesa_surface = PacketSerializer::read_optional(stream, |s| BiomeMesaSurfaceData::read(s));
        let capped_surface = PacketSerializer::read_optional(stream, |s| BiomeCappedSurfaceData::read(s));
        let noise_gradient_surface = PacketSerializer::read_optional(stream, |s| BiomeNoiseGradientSurfaceData::read(s));

        BiomeSurfaceBuilderData::new(surface_material, default_overworld_surface, swamp_surface, frozen_ocean_surface, the_end_surface, mesa_surface, capped_surface, noise_gradient_surface)
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::write_optional(stream, &self.surface_material, |s, v| v.write(s));
        stream.put_bool(self.default_overworld_surface);
        stream.put_bool(self.swamp_surface);
        stream.put_bool(self.frozen_ocean_surface);
        stream.put_bool(self.the_end_surface);
        PacketSerializer::write_optional(stream, &self.mesa_surface, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.capped_surface, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.noise_gradient_surface, |s, v| v.write(s));
    }
}
