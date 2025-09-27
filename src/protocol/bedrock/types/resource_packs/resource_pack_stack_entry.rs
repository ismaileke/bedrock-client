use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct ResourcePackStackEntry {
    pub pack_id: String,
    pub version: String,
    pub sub_pack_name: String
}

impl ResourcePackStackEntry {
    pub fn read(stream: &mut Stream) -> ResourcePackStackEntry {

        let mut length = stream.get_unsigned_var_int();
        let pack_id = String::from_utf8(stream.get(length).unwrap()).unwrap();

        length = stream.get_unsigned_var_int();
        let version = String::from_utf8(stream.get(length).unwrap()).unwrap();

        length = stream.get_unsigned_var_int();
        let sub_pack_name = String::from_utf8(stream.get(length).unwrap()).unwrap();

        ResourcePackStackEntry{ pack_id, version, sub_pack_name }
    }
}