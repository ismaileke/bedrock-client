use crate::protocol::bedrock::types::device_os::DeviceOS;
use crate::protocol::bedrock::types::skin::skin_data::SkinData;
use crate::protocol::bedrock::types::skin::skin_image::SkinImage;
use crate::utils::color::Color;

#[derive(Debug)]
pub struct PlayerListEntry {
    pub uuid: String,
    pub actor_unique_id: i64,
    pub username: String,
    pub skin_data: SkinData,
    pub xbox_user_id: String,
    pub platform_chat_id: String,
    pub build_platform: u32,
    pub is_teacher: bool,
    pub is_host: bool,
    pub is_sub_client: bool,
    pub color: Option<Color>
}


impl PlayerListEntry {

    pub fn create_removal_entry(uuid: String) -> PlayerListEntry {
        PlayerListEntry{
            uuid,
            actor_unique_id: 0,
            username: String::new(),
            skin_data: SkinData::default(String::new(), String::new(), String::new(), SkinImage::new(0, 0, String::new())),
            xbox_user_id: String::new(),
            platform_chat_id: String::new(),
            build_platform: DeviceOS::UNKNOWN, //-1?
            is_teacher: false,
            is_host: false,
            is_sub_client: false,
            color: None,
        }
    }

    pub fn create_addition_entry(
        uuid: String,
        actor_unique_id: i64,
        username: String,
        skin_data: SkinData,
        xbox_user_id: String,
        platform_chat_id: String,
        build_platform: u32,
        is_teacher: bool,
        is_host: bool,
        is_sub_client: bool,
        color: Option<Color>,
    ) -> PlayerListEntry {
        PlayerListEntry { uuid, actor_unique_id, username, skin_data, xbox_user_id, platform_chat_id, build_platform, is_teacher, is_host, is_sub_client, color }
    }
}