use crate::protocol::bedrock::player_list::PlayerList;
use crate::protocol::bedrock::types::device_os::DeviceOS;
use crate::protocol::bedrock::types::skin::skin_data::SkinData;
use crate::protocol::bedrock::types::skin::skin_image::SkinImage;
use crate::utils::color::Color;

#[derive(serde::Serialize, Debug)]
pub struct PlayerListEntry {
    pub action_type: u8,
    pub uuid: String,
    pub actor_unique_id: i64,
    pub username: String,
    pub xbox_user_id: String,
    pub platform_chat_id: String,
    pub build_platform: i32,
    pub skin_data: SkinData,
    pub is_teacher: bool,
    pub is_host: bool,
    pub is_sub_client: bool,
    pub color: Color,
}

impl PlayerListEntry {
    pub fn create_removal_entry(uuid: String) -> PlayerListEntry {
        PlayerListEntry {
            action_type: PlayerList::TYPE_REMOVE,
            uuid,
            actor_unique_id: 0,
            username: String::new(),
            xbox_user_id: String::new(),
            platform_chat_id: String::new(),
            build_platform: DeviceOS::UNKNOWN,
            skin_data: SkinData::default(
                String::new(),
                String::new(),
                String::new(),
                SkinImage::new(0, 0, vec![]),
            ),
            is_teacher: false,
            is_host: false,
            is_sub_client: false,
            color: Color::new(255, 255, 255, 255),
        }
    }
}
