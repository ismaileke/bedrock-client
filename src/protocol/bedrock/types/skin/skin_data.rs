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
    pub arm_size: u8,
    pub skin_color: i32,
    pub persona_pieces: Vec<PersonaSkinPiece>,
    pub piece_tint_colors: Vec<PersonaPieceTintColor>,
    pub is_verified: bool,
    pub premium: bool,
    pub persona: bool,
    pub persona_cape_on_classic: bool,
    pub is_primary_user: bool,
    pub is_override: bool,
    pub trusted_skin_flag: String,
    pub profile_hash: String
}

impl SkinData {
    pub const PIECE_TYPE_MAP: [(&'static str, i32); 27] = [
        ("persona_skeleton", PersonaSkinPiece::PIECE_TYPE_SKELETON),
        ("persona_body", PersonaSkinPiece::PIECE_TYPE_BODY),
        ("persona_skin", PersonaSkinPiece::PIECE_TYPE_SKIN),
        ("persona_bottom", PersonaSkinPiece::PIECE_TYPE_BOTTOM),
        ("persona_feet", PersonaSkinPiece::PIECE_TYPE_FEET),
        ("persona_dress", PersonaSkinPiece::PIECE_TYPE_DRESS),
        ("persona_top", PersonaSkinPiece::PIECE_TYPE_TOP),
        ("persona_high_pants", PersonaSkinPiece::PIECE_TYPE_HIGH_PANTS),
        ("persona_hands", PersonaSkinPiece::PIECE_TYPE_HANDS),
        ("persona_outerwear", PersonaSkinPiece::PIECE_TYPE_OUTERWEAR),
        ("persona_facial_hair", PersonaSkinPiece::PIECE_TYPE_FACIAL_HAIR),
        ("persona_mouth", PersonaSkinPiece::PIECE_TYPE_MOUTH),
        ("persona_eyes", PersonaSkinPiece::PIECE_TYPE_EYES),
        ("persona_hair", PersonaSkinPiece::PIECE_TYPE_HAIR),
        ("persona_hood", PersonaSkinPiece::PIECE_TYPE_HOOD),
        ("persona_back", PersonaSkinPiece::PIECE_TYPE_BACK),
        ("persona_face_accessory", PersonaSkinPiece::PIECE_TYPE_FACE_ACCESSORY),
        ("persona_head", PersonaSkinPiece::PIECE_TYPE_HEAD),
        ("persona_legs", PersonaSkinPiece::PIECE_TYPE_LEGS),
        ("persona_left_leg", PersonaSkinPiece::PIECE_TYPE_LEFT_LEG),
        ("persona_right_leg", PersonaSkinPiece::PIECE_TYPE_RIGHT_LEG),
        ("persona_arms", PersonaSkinPiece::PIECE_TYPE_ARMS),
        ("persona_left_arm", PersonaSkinPiece::PIECE_TYPE_LEFT_ARM),
        ("persona_right_arm", PersonaSkinPiece::PIECE_TYPE_RIGHT_ARM),
        ("persona_capes", PersonaSkinPiece::PIECE_TYPE_CAPES),
        ("persona_classic_skin", PersonaSkinPiece::PIECE_TYPE_CLASSIC_SKIN),
        ("persona_emote", PersonaSkinPiece::PIECE_TYPE_EMOTE)
    ];

    pub const ARM_SIZE_SLIM: u8 = 0;
    pub const ARM_SIZE_WIDE: u8 = 1;

    pub const TRUSTED_SKIN_FLAG_UNSET: &'static str = "unset";
    pub const TRUSTED_SKIN_FLAG_FALSE: &'static str = "false";
    pub const TRUSTED_SKIN_FLAG_TRUE: &'static str = "true";

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
            cape_image: Some(SkinImage::new(0, 0, vec![])),
            geometry_data: String::new(),
            geometry_data_engine_version: String::from("1.26.44"),
            animation_data: String::new(),
            cape_id: String::new(),
            full_skin_id: Some(Uuid::new_v4().to_string()),
            arm_size: Self::ARM_SIZE_WIDE,
            skin_color: 0,
            persona_pieces: vec![],
            piece_tint_colors: vec![],
            is_verified: true,
            premium: false,
            persona: false,
            persona_cape_on_classic: false,
            is_primary_user: true,
            is_override: true,
            trusted_skin_flag: Self::TRUSTED_SKIN_FLAG_UNSET.to_string(),
            profile_hash: "".to_string(),
        }
    }
}
