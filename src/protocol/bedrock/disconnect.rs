use binary_utils::binary::Stream;

pub struct Disconnect {
    pub reason: i32,
    pub skip_message: bool,
    pub message: Option<String>,
    pub filtered_message: Option<String>
}

pub fn decode(bytes: Vec<u8>) -> Disconnect {
    let mut stream = Stream::new(bytes, 0);

    let reason = stream.get_var_int();  // bunda da sıkıntı var gibi?
    let skip_message = stream.get_bool();
    let mut message: Option<String> = None;
    let mut filtered_message: Option<String> = None;

    if !skip_message {
        let mut length = stream.get_unsigned_var_int();
        message = Option::from(String::from_utf8(stream.get(length).unwrap()).unwrap());

        length = stream.get_unsigned_var_int();
        filtered_message = Option::from(String::from_utf8(stream.get(length).unwrap()).unwrap());
    }

    Disconnect { reason, skip_message, message, filtered_message }
}