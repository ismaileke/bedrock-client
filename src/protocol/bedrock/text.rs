use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;

pub const TYPE_RAW: u8 = 0;
pub const TYPE_CHAT: u8 = 1;
pub const TYPE_TRANSLATION: u8 = 2;
pub const TYPE_POPUP: u8 = 3;
pub const TYPE_JUKEBOX_POPUP: u8 = 4;
pub const TYPE_TIP: u8 = 5;
pub const TYPE_SYSTEM: u8 = 6;
pub const TYPE_WHISPER: u8 = 7;
pub const TYPE_ANNOUNCEMENT: u8 = 8;
pub const TYPE_JSON_WHISPER: u8 = 9;
pub const TYPE_JSON: u8 = 10;
pub const TYPE_JSON_ANNOUNCEMENT: u8 = 11;

pub struct Text {
    pub text_type: u8,
    pub needs_translation: bool,

    pub source_name: Option<String>,
    pub message: String,
    pub parameters: Option<Vec<String>>,

    pub xbox_uid: String,
    pub platform_chat_id: String,
    pub filtered_message: String,
}

pub fn new(text_type: u8, needs_translation: bool, source_name: Option<String>, message: String, parameters: Option<Vec<String>>, xbox_uid: String, platform_chat_id: String, filtered_message: String) -> Text {
    Text{ text_type, needs_translation, source_name, message, parameters, xbox_uid, platform_chat_id, filtered_message }
}

impl Text {
    pub fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::Text) as u32);

        stream.put_byte(self.text_type);
        stream.put_bool(self.needs_translation);
        match self.text_type {
            TYPE_CHAT | TYPE_WHISPER | TYPE_ANNOUNCEMENT => {
                if let Some(source_name) = self.source_name.clone() {
                    stream.put_unsigned_var_int(source_name.len() as u32);
                    stream.put(source_name.into_bytes());
                }
                stream.put_unsigned_var_int(self.message.len() as u32);
                stream.put(self.message.clone().into_bytes());

            },
            TYPE_RAW | TYPE_TIP | TYPE_SYSTEM | TYPE_JSON | TYPE_JSON_WHISPER | TYPE_JSON_ANNOUNCEMENT => {
                stream.put_unsigned_var_int(self.message.len() as u32);
                stream.put(self.message.clone().into_bytes());
            },
            TYPE_TRANSLATION | TYPE_POPUP | TYPE_JUKEBOX_POPUP => {
                stream.put_unsigned_var_int(self.message.len() as u32);
                stream.put(self.message.clone().into_bytes());
                if let Some(parameters) = self.parameters.clone() {
                    stream.put_unsigned_var_int(parameters.len() as u32);
                    for parameter in parameters {
                        stream.put_unsigned_var_int(parameter.len() as u32);
                        stream.put(parameter.into_bytes());
                    }
                }
            }
            _ => {}
        }
        stream.put_unsigned_var_int(self.xbox_uid.len() as u32);
        stream.put(self.xbox_uid.clone().into_bytes());
        stream.put_unsigned_var_int(self.platform_chat_id.len() as u32);
        stream.put(self.platform_chat_id.clone().into_bytes());
        stream.put_unsigned_var_int(self.filtered_message.len() as u32);
        stream.put(self.filtered_message.clone().into_bytes());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    pub fn debug(&self) {
        if let Some(source_name) = self.source_name.clone() {
            println!("Source Name: {}", source_name);
        }
        println!("Message: {}", self.message);
        if let Some(parameters) = self.parameters.clone() {
            println!("Parameters: {}", parameters.join(" "));
        }
    }
}

pub fn decode(bytes: Vec<u8>) -> Text {
    let mut stream = Stream::new(bytes, 0);

    let text_type = stream.get_byte();
    let needs_translation = stream.get_bool();

    let mut source_name: Option<String> = None;
    let mut message = String::new();
    let mut parameters: Option<Vec<String>> = None;
    match text_type {
        TYPE_CHAT | TYPE_WHISPER | TYPE_ANNOUNCEMENT => {
            let mut length = stream.get_unsigned_var_int();
            source_name = Option::from(String::from_utf8(stream.get(length).unwrap()).unwrap());
            length = stream.get_unsigned_var_int();
            message = String::from_utf8(stream.get(length).unwrap()).unwrap();

        },
        TYPE_RAW | TYPE_TIP | TYPE_SYSTEM | TYPE_JSON | TYPE_JSON_WHISPER | TYPE_JSON_ANNOUNCEMENT => {
            let length = stream.get_unsigned_var_int();
            message = String::from_utf8(stream.get(length).unwrap()).unwrap();
        },
        TYPE_TRANSLATION | TYPE_POPUP | TYPE_JUKEBOX_POPUP => {
            let mut length = stream.get_unsigned_var_int();
            message = String::from_utf8(stream.get(length).unwrap()).unwrap();
            length = stream.get_unsigned_var_int();
            let mut params = Vec::new();
            for _ in 0..length {
                let length = stream.get_unsigned_var_int();
                let parameter = String::from_utf8(stream.get(length).unwrap()).unwrap();
                params.push(parameter);
            }
            parameters = Option::from(params);
        }
        _ => {}
    }

    let mut length = stream.get_unsigned_var_int();
    let xbox_uid = String::from_utf8(stream.get(length).unwrap()).unwrap();

    length = stream.get_unsigned_var_int();
    let platform_chat_id = String::from_utf8(stream.get(length).unwrap()).unwrap();

    length = stream.get_unsigned_var_int();
    let filtered_message = String::from_utf8(stream.get(length).unwrap()).unwrap();

    Text { text_type, needs_translation, source_name, message, parameters, xbox_uid, platform_chat_id, filtered_message }
}