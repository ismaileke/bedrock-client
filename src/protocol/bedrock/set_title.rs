use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetTitle {
    pub title_type: i32,
    pub text: String,
    pub fade_in_time: i32,
    pub stay_time: i32,
    pub fade_out_time: i32,
    pub xuid: String,
    pub platform_online_id: String,
    pub filtered_title_text: String,
}

impl Packet for SetTitle {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetTitle.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.title_type);
        PacketSerializer::put_string(stream, &self.text);
        stream.put_var_i32(self.fade_in_time);
        stream.put_var_i32(self.stay_time);
        stream.put_var_i32(self.fade_out_time);
        PacketSerializer::put_string(stream, &self.xuid);
        PacketSerializer::put_string(stream, &self.platform_online_id);
        PacketSerializer::put_string(stream, &self.filtered_title_text);
    }

    fn decode(stream: &mut Reader) -> SetTitle {
        let title_type = stream.get_var_i32();
        let text = PacketSerializer::get_string(stream);
        let fade_in_time = stream.get_var_i32();
        let stay_time = stream.get_var_i32();
        let fade_out_time = stream.get_var_i32();
        let xuid = PacketSerializer::get_string(stream);
        let platform_online_id = PacketSerializer::get_string(stream);
        let filtered_title_text = PacketSerializer::get_string(stream);

        SetTitle {
            title_type,
            text,
            fade_in_time,
            stay_time,
            fade_out_time,
            xuid,
            platform_online_id,
            filtered_title_text,
        }
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
