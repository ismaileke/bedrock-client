use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SpawnSettings {
    pub biome_type: u16,
    pub biome_name: String,
    pub dimension_id: i32,
}

impl SpawnSettings {
    pub const BIOME_TYPE_DEFAULT: u16 = 0;
    pub const BIOME_TYPE_USER_DEFINED: u16 = 1;

    pub fn read(stream: &mut Stream) -> SpawnSettings {
        let biome_type = stream.get_u16_le();
        let biome_name = PacketSerializer::get_string(stream);
        let dimension_id = stream.get_var_i32();

        SpawnSettings {
            biome_type,
            biome_name,
            dimension_id,
        }
    }
}
