use crate::protocol::bedrock::types::skin::persona_piece_tint_color::PersonaPieceTintColor;
use crate::protocol::bedrock::types::skin::persona_skin_piece::PersonaSkinPiece;
use crate::protocol::bedrock::types::skin::skin_animation::SkinAnimation;
use crate::protocol::bedrock::types::skin::skin_image::SkinImage;
use uuid::Uuid;

#[derive(serde::Serialize, Debug)]
pub struct SkinData {
    pub skin_id: String,
    pub play_fab_id: String,
    pub resource_patch: String,
    pub skin_image: SkinImage,
    pub animations: Vec<SkinAnimation>,
    pub cape_image: Option<SkinImage>,
    pub geometry_data: String,
    pub geometry_data_engine_version: String,
    pub animation_data: String,
    pub cape_id: String,
    pub full_skin_id: Option<String>,
    pub arm_size: String,
    pub skin_color: String,
    pub persona_pieces: Vec<PersonaSkinPiece>,
    pub piece_tint_colors: Vec<PersonaPieceTintColor>,
    pub is_verified: bool,
    pub premium: bool,
    pub persona: bool,
    pub persona_cape_on_classic: bool,
    pub is_primary_user: bool,
    pub is_override: bool,
}

impl SkinData {
    pub const ARM_SIZE_SLIM: &'static str = "slim";
    pub const ARM_SIZE_WIDE: &'static str = "wide";

    pub fn default(
        skin_id: String,
        play_fab_id: String,
        resource_patch: String,
        skin_image: SkinImage,
    ) -> SkinData {
        SkinData {
            skin_id,
            play_fab_id,
            resource_patch,
            skin_image,
            animations: vec![],
            cape_image: Some(SkinImage::new(0, 0, String::new())),
            geometry_data: String::new(),
            geometry_data_engine_version: String::from("1.21.124"),
            animation_data: String::new(),
            cape_id: String::new(),
            full_skin_id: Some(Uuid::new_v4().to_string()),
            arm_size: String::from(Self::ARM_SIZE_WIDE.to_string()),
            skin_color: String::new(),
            persona_pieces: vec![],
            piece_tint_colors: vec![],
            is_verified: true,
            premium: false,
            persona: false,
            persona_cape_on_classic: false,
            is_primary_user: true,
            is_override: true,
        }
    }
}
