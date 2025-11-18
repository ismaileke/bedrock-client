use std::any::Any;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct Text {
    pub text_type: u8,
    pub needs_translation: bool,
    pub source_name: Option<String>,
    pub message: String,
    pub parameters: Option<Vec<String>>,
    pub xbox_uid: String,
    pub platform_chat_id: String,
    pub filtered_message: String
}

pub fn new(text_type: u8, needs_translation: bool, source_name: Option<String>, message: String, parameters: Option<Vec<String>>, xbox_uid: String, platform_chat_id: String, filtered_message: String) -> Text {
    Text{ text_type, needs_translation, source_name, message, parameters, xbox_uid, platform_chat_id, filtered_message }
}

impl Packet for Text {
    fn id(&self) -> u16 {
        BedrockPacketType::IDText.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.text_type);
        stream.put_bool(self.needs_translation);
        match self.text_type {
            Text::TYPE_CHAT | Text::TYPE_WHISPER | Text::TYPE_ANNOUNCEMENT => {
                if let Some(source_name) = self.source_name.clone() {
                    PacketSerializer::put_string(&mut stream, source_name);
                }
                PacketSerializer::put_string(&mut stream, self.message.clone());
            },
            Text::TYPE_RAW | Text::TYPE_TIP | Text::TYPE_SYSTEM | Text::TYPE_JSON | Text::TYPE_JSON_WHISPER | Text::TYPE_JSON_ANNOUNCEMENT => {
                PacketSerializer::put_string(&mut stream, self.message.clone());
            },
            Text::TYPE_TRANSLATION | Text::TYPE_POPUP | Text::TYPE_JUKEBOX_POPUP => {
                PacketSerializer::put_string(&mut stream, self.message.clone());
                if let Some(parameters) = self.parameters.clone() {
                    stream.put_var_u32(parameters.len() as u32);
                    for parameter in parameters {
                        PacketSerializer::put_string(&mut stream, parameter);
                    }
                }
            }
            _ => {}
        }
        PacketSerializer::put_string(&mut stream, self.xbox_uid.clone());
        PacketSerializer::put_string(&mut stream, self.platform_chat_id.clone());
        PacketSerializer::put_string(&mut stream, self.filtered_message.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> Text {
        let mut stream = Stream::new(bytes, 0);

        let text_type = stream.get_byte();
        let needs_translation = stream.get_bool();

        let mut source_name: Option<String> = None;
        let mut message = String::new();
        let mut parameters: Option<Vec<String>> = None;
        match text_type {
            Text::TYPE_CHAT | Text::TYPE_WHISPER | Text::TYPE_ANNOUNCEMENT => {
                source_name = Option::from(PacketSerializer::get_string(&mut stream));
                message = PacketSerializer::get_string(&mut stream);

            },
            Text::TYPE_RAW | Text::TYPE_TIP | Text::TYPE_SYSTEM | Text::TYPE_JSON | Text::TYPE_JSON_WHISPER | Text::TYPE_JSON_ANNOUNCEMENT => {
                message = PacketSerializer::get_string(&mut stream);
            },
            Text::TYPE_TRANSLATION | Text::TYPE_POPUP | Text::TYPE_JUKEBOX_POPUP => {
                message = PacketSerializer::get_string(&mut stream);
                let length = stream.get_var_u32();
                let mut params = Vec::new();
                for _ in 0..length {
                    let parameter = PacketSerializer::get_string(&mut stream);
                    params.push(parameter);
                }
                parameters = Option::from(params);
            }
            _ => {}
        }

        let xbox_uid = PacketSerializer::get_string(&mut stream);
        let platform_chat_id = PacketSerializer::get_string(&mut stream);
        let filtered_message = PacketSerializer::get_string(&mut stream);

        Text { text_type, needs_translation, source_name, message, parameters, xbox_uid, platform_chat_id, filtered_message }
    }

    fn debug(&self) {
        if let Some(source_name) = self.source_name.clone() {
            println!("Source Name: {}", source_name);
        }
        println!("Message: {}", self.message);
        if let Some(parameters) = self.parameters.clone() {
            println!("Parameters: {}", parameters.join(" "));
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Text {
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
}
