use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SpawnSettings {
    pub biome_type: u16,
    pub biome_name: String,
    pub dimension_id: i32,
}

impl SpawnSettings {
    pub const BIOME_TYPE_DEFAULT: u16 = 0;
    pub const BIOME_TYPE_USER_DEFINED: u16 = 1;

    pub fn read(stream: &mut Reader) -> SpawnSettings {
        let biome_type = stream.get_u16_le();
        let biome_name = PacketSerializer::get_string(stream);
        let dimension_id = stream.get_var_i32();

        SpawnSettings { biome_type, biome_name, dimension_id }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u16_le(self.biome_type);
        PacketSerializer::put_string(stream, &self.biome_name);
        stream.put_var_i32(self.dimension_id);
    }
}
