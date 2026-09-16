use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct DimensionDataEntry {
    pub minimum_y: i32,
    pub height_range: i32,
    pub generator: i32,
    pub dimension_type: i32,
    pub pack_id: String,
    pub default_biome: String
}

impl DimensionDataEntry {
    pub fn new(minimum_y: i32, height_range: i32, generator: i32, dimension_type: i32, pack_id: String, default_biome: String) -> DimensionDataEntry {
        DimensionDataEntry { minimum_y, height_range, generator, dimension_type, pack_id, default_biome }
    }

    pub fn read(stream: &mut Reader) -> DimensionDataEntry {
        let minimum_y = stream.get_var_i32();
        let height_range = stream.get_var_i32();
        let generator = stream.get_var_i32();
        let dimension_type = stream.get_var_i32();
        let pack_id = PacketSerializer::get_uuid(stream);
        let default_biome = PacketSerializer::get_string(stream);

        DimensionDataEntry { minimum_y, height_range, generator, dimension_type, pack_id, default_biome }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_i32(self.minimum_y);
        stream.put_var_i32(self.height_range);
        stream.put_var_i32(self.generator);
        PacketSerializer::put_uuid(stream, &self.pack_id);
        PacketSerializer::put_string(stream, &self.default_biome);
    }
}
