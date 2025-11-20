use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct SetTitle {
    pub title_type: i32,
    pub text: String,
    pub fade_in_time: i32,
    pub stay_time: i32,
    pub fade_out_time: i32,
    pub xuid: String,
    pub platform_online_id: String,
    pub filtered_title_text: String
}

pub fn new(title_type: i32, text: String, fade_in_time: i32, stay_time: i32, fade_out_time: i32, xuid: String, platform_online_id: String, filtered_title_text: String) -> SetTitle {
    SetTitle { title_type, text, fade_in_time, stay_time, fade_out_time, xuid, platform_online_id, filtered_title_text }
}

impl Packet for SetTitle {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetTitle.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.title_type);
        PacketSerializer::put_string(&mut stream, self.text.clone());
        stream.put_var_i32(self.fade_in_time);
        stream.put_var_i32(self.stay_time);
        stream.put_var_i32(self.fade_out_time);
        PacketSerializer::put_string(&mut stream, self.xuid.clone());
        PacketSerializer::put_string(&mut stream, self.platform_online_id.clone());
        PacketSerializer::put_string(&mut stream, self.filtered_title_text.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SetTitle {
        let title_type = stream.get_var_i32();
        let text = PacketSerializer::get_string(stream);
        let fade_in_time = stream.get_var_i32();
        let stay_time = stream.get_var_i32();
        let fade_out_time = stream.get_var_i32();
        let xuid = PacketSerializer::get_string(stream);
        let platform_online_id = PacketSerializer::get_string(stream);
        let filtered_title_text = PacketSerializer::get_string(stream);

        SetTitle { title_type, text, fade_in_time, stay_time, fade_out_time, xuid, platform_online_id, filtered_title_text }
    }
    
    fn debug(&self) {
        println!("Title Type: {}", self.title_type);
        println!("Text: {}", self.text);
        println!("Fade In Time: {}", self.fade_in_time);
        println!("Stay Time: {}", self.stay_time);
        println!("Fade Out Time: {}", self.fade_out_time);
        println!("XUID: {}", self.xuid);
        println!("Platform Online ID: {}", self.platform_online_id);
        println!("Filtered Title Text: {}", self.filtered_title_text);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SetTitle {
    pub const TYPE_CLEAR_TITLE: i32 = 0;
    pub const TYPE_RESET_TITLE: i32 = 1;
    pub const TYPE_SET_TITLE: i32 = 2;
    pub const TYPE_SET_SUBTITLE: i32 = 3;
    pub const TYPE_SET_ACTIONBAR_MESSAGE: i32 = 4;
    pub const TYPE_SET_ANIMATION_TIMES: i32 = 5;
    pub const TYPE_SET_TITLE_JSON: i32 = 6;
    pub const TYPE_SET_SUBTITLE_JSON: i32 = 7;
    pub const TYPE_SET_ACTIONBAR_MESSAGE_JSON: i32 = 8;
}
