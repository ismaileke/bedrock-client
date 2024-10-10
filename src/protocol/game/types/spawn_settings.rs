use binary_utils::binary::Stream;

pub struct SpawnSettings {
    biome_type: u16,
    biome_name: String,
    dimension_id: i32
}

impl SpawnSettings {
    pub fn read(stream: &mut Stream) -> SpawnSettings {
        let biome_type = stream.get_l_short();

        let length = stream.get_unsigned_var_int();
        let biome_name = String::from_utf8(stream.get(length).unwrap()).unwrap();

        let dimension_id = stream.get_var_int();

        SpawnSettings{ biome_type, biome_name, dimension_id }
    }
}