use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::biome::chunkgen::biome_capped_surface_data::BiomeCappedSurfaceData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_climate_data::BiomeClimateData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_consolidated_features_data::BiomeConsolidatedFeaturesData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_legacy_world_gen_rules_data::BiomeLegacyWorldGenRulesData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_mesa_surface_data::BiomeMesaSurfaceData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_mountain_params_data::BiomeMountainParamsData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_multinoise_gen_rules_data::BiomeMultinoiseGenRulesData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_overworld_gen_rules_data::BiomeOverworldGenRulesData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_surface_material_adjustment_data::BiomeSurfaceMaterialAdjustmentData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_surface_material_data::BiomeSurfaceMaterialData;

#[derive(Debug)]
pub struct BiomeDefinitionChunkGenData {
    climate: Option<BiomeClimateData>,
    consolidated_features: Option<BiomeConsolidatedFeaturesData>,
    mountain_params: Option<BiomeMountainParamsData>,
    surface_material_adjustment: Option<BiomeSurfaceMaterialAdjustmentData>,
    surface_material: Option<BiomeSurfaceMaterialData>,
    swamp_surface: bool,
    frozen_ocean_surface: bool,
    the_end_surface: bool,
    mesa_surface: Option<BiomeMesaSurfaceData>,
    capped_surface: Option<BiomeCappedSurfaceData>,
    overworld_gen_rules: Option<BiomeOverworldGenRulesData>,
    multinoise_gen_rules: Option<BiomeMultinoiseGenRulesData>,
    legacy_world_gen_rules: Option<BiomeLegacyWorldGenRulesData>
}

impl BiomeDefinitionChunkGenData {
    pub fn new(
        climate: Option<BiomeClimateData>,
        consolidated_features: Option<BiomeConsolidatedFeaturesData>,
        mountain_params: Option<BiomeMountainParamsData>,
        surface_material_adjustment: Option<BiomeSurfaceMaterialAdjustmentData>,
        surface_material: Option<BiomeSurfaceMaterialData>,
        swamp_surface: bool,
        frozen_ocean_surface: bool,
        the_end_surface: bool,
        mesa_surface: Option<BiomeMesaSurfaceData>,
        capped_surface: Option<BiomeCappedSurfaceData>,
        overworld_gen_rules: Option<BiomeOverworldGenRulesData>,
        multinoise_gen_rules: Option<BiomeMultinoiseGenRulesData>,
        legacy_world_gen_rules: Option<BiomeLegacyWorldGenRulesData>
    ) -> Self {
        BiomeDefinitionChunkGenData{
            climate,
            consolidated_features,
            mountain_params,
            surface_material_adjustment,
            surface_material,
            swamp_surface,
            frozen_ocean_surface,
            the_end_surface,
            mesa_surface,
            capped_surface,
            overworld_gen_rules,
            multinoise_gen_rules,
            legacy_world_gen_rules
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeDefinitionChunkGenData {
        let climate = PacketSerializer::read_optional(stream, |s| BiomeClimateData::read(s));
        let consolidated_features = PacketSerializer::read_optional(stream, |s| BiomeConsolidatedFeaturesData::read(s));
        let mountain_params = PacketSerializer::read_optional(stream, |s| BiomeMountainParamsData::read(s));
        let surface_material_adjustment = PacketSerializer::read_optional(stream, |s| BiomeSurfaceMaterialAdjustmentData::read(s));
        let surface_material = PacketSerializer::read_optional(stream, |s| BiomeSurfaceMaterialData::read(s));
        let swamp_surface = stream.get_bool();
        let frozen_ocean_surface = stream.get_bool();
        let the_end_surface = stream.get_bool();
        let mesa_surface = PacketSerializer::read_optional(stream, |s| BiomeMesaSurfaceData::read(s));
        let capped_surface = PacketSerializer::read_optional(stream, |s| BiomeCappedSurfaceData::read(s));
        let overworld_gen_rules = PacketSerializer::read_optional(stream, |s| BiomeOverworldGenRulesData::read(s));
        let multinoise_gen_rules = PacketSerializer::read_optional(stream, |s| BiomeMultinoiseGenRulesData::read(s));
        let legacy_world_gen_rules = PacketSerializer::read_optional(stream, |s| BiomeLegacyWorldGenRulesData::read(s));

        BiomeDefinitionChunkGenData::new(
            climate,
            consolidated_features,
            mountain_params,
            surface_material_adjustment,
            surface_material,
            swamp_surface,
            frozen_ocean_surface,
            the_end_surface,
            mesa_surface,
            capped_surface,
            overworld_gen_rules,
            multinoise_gen_rules,
            legacy_world_gen_rules
        )
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::write_optional(stream, &self.climate, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.consolidated_features, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.mountain_params, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.surface_material_adjustment, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.surface_material, |s, v| v.write(s));
        stream.put_bool(self.swamp_surface);
        stream.put_bool(self.frozen_ocean_surface);
        stream.put_bool(self.the_end_surface);
        PacketSerializer::write_optional(stream, &self.mesa_surface, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.capped_surface, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.overworld_gen_rules, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.multinoise_gen_rules, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.legacy_world_gen_rules, |s, v| v.write(s));
    }
}