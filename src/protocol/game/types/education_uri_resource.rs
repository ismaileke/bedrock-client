use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct EducationUriResource {
    pub button_name: String,
    pub link_uri: String
}

impl EducationUriResource {
    pub fn read(stream: &mut Stream) -> EducationUriResource {
        let mut length = stream.get_unsigned_var_int();
        let button_name = String::from_utf8(stream.get(length).unwrap()).unwrap();
        length = stream.get_unsigned_var_int();
        let link_uri = String::from_utf8(stream.get(length).unwrap()).unwrap();

        EducationUriResource{ button_name, link_uri }
    }
}