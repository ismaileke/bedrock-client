use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct DimensionDataEntry {
    pub name: String,
    pub max_height: i32,
    pub min_height: i32,
    pub generator: i32,
    pub dimension_type: i32,
    pub pack_id: String
}

impl DimensionDataEntry {
    pub fn new(name: String, max_height: i32, min_height: i32, generator: i32, dimension_type: i32, pack_id: String) -> DimensionDataEntry {
        DimensionDataEntry { name, max_height, min_height, generator, dimension_type, pack_id }
    }

    pub fn read(stream: &mut Reader) -> DimensionDataEntry {
        let name = PacketSerializer::get_string(stream);
        let max_height = stream.get_var_i32();
        let min_height = stream.get_var_i32();
        let generator = stream.get_var_i32();
        let dimension_type = stream.get_var_i32();
        let pack_id = PacketSerializer::get_uuid(stream);

        DimensionDataEntry { name, max_height, min_height, generator, dimension_type, pack_id }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.name);
        stream.put_var_i32(self.max_height);
        stream.put_var_i32(self.min_height);
        stream.put_var_i32(self.generator);
        PacketSerializer::put_uuid(stream, &self.pack_id);
    }
}
