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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.title_type);
        PacketSerializer::put_string(&mut stream, self.text.clone());
        stream.put_var_int(self.fade_in_time);
        stream.put_var_int(self.stay_time);
        stream.put_var_int(self.fade_out_time);
        PacketSerializer::put_string(&mut stream, self.xuid.clone());
        PacketSerializer::put_string(&mut stream, self.platform_online_id.clone());
        PacketSerializer::put_string(&mut stream, self.filtered_title_text.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetTitle {
        let mut stream = Stream::new(bytes, 0);

        let title_type = stream.get_var_int();
        let text = PacketSerializer::get_string(&mut stream);
        let fade_in_time = stream.get_var_int();
        let stay_time = stream.get_var_int();
        let fade_out_time = stream.get_var_int();
        let xuid = PacketSerializer::get_string(&mut stream);
        let platform_online_id = PacketSerializer::get_string(&mut stream);
        let filtered_title_text = PacketSerializer::get_string(&mut stream);

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
