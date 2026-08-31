#[derive(serde::Serialize, Debug)]
pub struct PersonaPieceTintColor {
    pub piece_type: String,
    pub colors: Vec<i32>,
}

impl PersonaPieceTintColor {
    pub const PIECE_TYPE_PERSONA_EYES: &'static str = "persona_eyes";
    pub const PIECE_TYPE_PERSONA_HAIR: &'static str = "persona_hair";
    pub const PIECE_TYPE_PERSONA_MOUTH: &'static str = "persona_mouth";

    pub const COLOR_COUNT: u32 = 4;

    pub fn new(piece_type: String, colors: Vec<i32>) -> PersonaPieceTintColor {
        if colors.len() != Self::COLOR_COUNT as usize{
            panic!("Expected exactly {} colors, got {}", Self::COLOR_COUNT, colors.len());
        }
        PersonaPieceTintColor { piece_type, colors }
    }

    pub fn piece_type(&self) -> &String {
        &self.piece_type
    }

    pub fn colors(&self) -> &Vec<i32> {
        &self.colors
    }
}
