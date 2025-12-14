use crate::protocol::bedrock::types::skin::skin_image::SkinImage;

#[derive(serde::Serialize, Debug)]
pub struct SkinAnimation {
    image: SkinImage,
    animation_type: u32,
    frames: f32,
    expression_type: u32,
}

impl SkinAnimation {
    pub const TYPE_HEAD: u32 = 1;
    pub const TYPE_BODY_32: u32 = 2;
    pub const TYPE_BODY_64: u32 = 3;

    pub const EXPRESSION_LINEAR: u32 = 0; //???
    pub const EXPRESSION_BLINKING: u32 = 1;

    pub fn new(
        image: SkinImage,
        animation_type: u32,
        frames: f32,
        expression_type: u32,
    ) -> SkinAnimation {
        SkinAnimation {
            image,
            animation_type,
            frames,
            expression_type,
        }
    }

    pub fn image(&self) -> &SkinImage {
        &self.image
    }

    pub fn animation_type(&self) -> u32 {
        self.animation_type
    }

    pub fn frames(&self) -> f32 {
        self.frames
    }

    pub fn expression_type(&self) -> u32 {
        self.expression_type
    }
}
