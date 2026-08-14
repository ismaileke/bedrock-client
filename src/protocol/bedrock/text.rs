use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct Text {
    pub text_type: u8,
    pub needs_translation: bool,
    pub source_name: Option<String>,
    pub message: String,
    pub parameters: Option<Vec<String>>,
    pub xbox_uid: String,
    pub platform_chat_id: String,
    pub filtered_message: Option<String>,
}

impl Packet for Text {
    fn id(&self) -> u16 {
        BedrockPacketType::IDText.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_bool(self.needs_translation);
        match self.text_type {
            Text::TYPE_RAW | Text::TYPE_TIP | Text::TYPE_SYSTEM | Text::TYPE_JSON_WHISPER | Text::TYPE_JSON_ANNOUNCEMENT | Text::TYPE_JSON => {
                stream.put_u8(Text::CATEGORY_MESSAGE_ONLY);
            },
            Text::TYPE_CHAT | Text::TYPE_WHISPER | Text::TYPE_ANNOUNCEMENT => {
                stream.put_u8(Text::CATEGORY_AUTHORED_MESSAGE);
            },
            _ => {
                stream.put_u8(Text::CATEGORY_MESSAGE_WITH_PARAMETERS);
            }
        }
        stream.put_u8(self.text_type);
        match self.text_type {
            Text::TYPE_CHAT | Text::TYPE_WHISPER | Text::TYPE_ANNOUNCEMENT => {
                if let Some(source_name) = &self.source_name {
                    PacketSerializer::put_string(stream, if source_name == "" { " " } else { source_name });
                }
                PacketSerializer::put_string(stream, if self.message.clone() == "" { " " } else { &self.message });
            }
            Text::TYPE_RAW | Text::TYPE_TIP | Text::TYPE_SYSTEM | Text::TYPE_JSON | Text::TYPE_JSON_WHISPER | Text::TYPE_JSON_ANNOUNCEMENT => {
                PacketSerializer::put_string(stream, if self.message.clone() == "" { " " } else { &self.message });
            }
            Text::TYPE_TRANSLATION | Text::TYPE_POPUP | Text::TYPE_JUKEBOX_POPUP => {
                PacketSerializer::put_string(stream, if self.message.clone() == "" { " " } else { &self.message });
                if let Some(parameters) = self.parameters.clone() {
                    stream.put_var_u32(parameters.len() as u32);
                    for parameter in &parameters {
                        PacketSerializer::put_string(stream, parameter);
                    }
                }
            }
            _ => {}
        }
        PacketSerializer::put_string(stream, &self.xbox_uid);
        PacketSerializer::put_string(stream, &self.platform_chat_id);
        PacketSerializer::write_optional(stream, &self.filtered_message, |s, v| PacketSerializer::put_string(s, v));
    }

    fn decode(stream: &mut Reader) -> Text {
        let needs_translation = stream.get_bool();
        let category = stream.get_u8();
        let text_type = stream.get_u8();
        let mut source_name: Option<String> = None;
        let mut message = String::new();
        let mut parameters: Option<Vec<String>> = None;
        match text_type {
            Text::TYPE_CHAT | Text::TYPE_WHISPER | Text::TYPE_ANNOUNCEMENT => {
                if category != Text::CATEGORY_AUTHORED_MESSAGE {
                    panic!("Decoded TextPacket has invalid structure: type {} requires category CATEGORY_AUTHORED_MESSAGE", text_type);
                }
                source_name = Option::from(PacketSerializer::get_string(stream));
                message = PacketSerializer::get_string(stream);
            }
            Text::TYPE_RAW | Text::TYPE_TIP | Text::TYPE_SYSTEM | Text::TYPE_JSON | Text::TYPE_JSON_WHISPER | Text::TYPE_JSON_ANNOUNCEMENT => {
                if category != Text::CATEGORY_MESSAGE_ONLY {
                    panic!("Decoded TextPacket has invalid structure: type {} requires category CATEGORY_MESSAGE_ONLY", text_type);
                }
                message = PacketSerializer::get_string(stream);
            }
            Text::TYPE_TRANSLATION | Text::TYPE_POPUP | Text::TYPE_JUKEBOX_POPUP => {
                if category != Text::CATEGORY_MESSAGE_WITH_PARAMETERS {
                    panic!("Decoded TextPacket has invalid structure: type {} requires category CATEGORY_MESSAGE_WITH_PARAMETERS", text_type);
                }
                message = PacketSerializer::get_string(stream);
                let length = stream.get_var_u32();
                let mut params = Vec::new();
                for _ in 0..length {
                    let parameter = PacketSerializer::get_string(stream);
                    params.push(parameter);
                }
                parameters = Option::from(params);
            }
            _ => {}
        }

        let xbox_uid = PacketSerializer::get_string(stream);
        let platform_chat_id = PacketSerializer::get_string(stream);
        let filtered_message = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));

        Text {
            text_type,
            needs_translation,
            source_name,
            message,
            parameters,
            xbox_uid,
            platform_chat_id,
            filtered_message,
        }
    }
}

impl Text {
    pub const CATEGORY_MESSAGE_ONLY: u8 = 0;
    pub const CATEGORY_AUTHORED_MESSAGE: u8 = 1;
    pub const CATEGORY_MESSAGE_WITH_PARAMETERS: u8 = 2;

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

    pub fn assert_string(received_value: String, required_value: &str) {
        if received_value.as_str() != required_value {
            panic!(
                "Decoded TextPacket has invalid structure: expected '{}', got '{}'",
                required_value,
                received_value
            );
        }
    }
}
