use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::biome::chunkgen::biome_element_data::BiomeElementData;

#[derive(Debug)]
pub struct BiomeSurfaceMaterialAdjustmentData {
    pub adjustments: Vec<BiomeElementData>
}

impl BiomeSurfaceMaterialAdjustmentData {
    pub fn new(adjustments: Vec<BiomeElementData>,) -> Self {
        BiomeSurfaceMaterialAdjustmentData{ adjustments }
    }

    pub fn read(stream: &mut Stream) -> BiomeSurfaceMaterialAdjustmentData {
        let mut adjustments = Vec::new();
        let count = stream.get_unsigned_var_int();
        for _ in 0..count {
            adjustments.push(BiomeElementData::read(stream));
        }

        BiomeSurfaceMaterialAdjustmentData::new(adjustments)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_unsigned_var_int(self.adjustments.len() as u32);
        for adjustment in &self.adjustments {
            adjustment.write(stream);
        }
    }
}