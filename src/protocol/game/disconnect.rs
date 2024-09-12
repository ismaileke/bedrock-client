use binary_utils::binary::Stream;

pub struct Disconnect {
    pub reason: i32,
    pub skip_message: bool,
    pub message: Option<String>,
    pub filtered_message: Option<String>
}

pub fn decode(bytes: Vec<u8>) -> Disconnect {
    let mut stream = Stream::new(bytes, 0);

    let reason = stream.get_var_int();//bunda da sıkıntı var gibi?
    let skip_message = stream.get_bool();

    if skip_message {
        let mut length = stream.get_unsigned_var_int();
        let message = String::from_utf8(stream.get(length).unwrap()).unwrap();
        length = stream.get_unsigned_var_int();
        let filtered_message = String::from_utf8(stream.get(length).unwrap()).unwrap();
        return Disconnect { reason, skip_message, message: Option::from(message), filtered_message: Option::from(filtered_message) };
    }

    Disconnect { reason, skip_message, message: None, filtered_message: None }
}