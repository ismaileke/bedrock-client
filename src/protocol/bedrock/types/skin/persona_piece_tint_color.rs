#[derive(serde::Serialize, Debug)]
pub struct PersonaPieceTintColor {
    piece_type: String,
    colors: Vec<String>,
}

impl PersonaPieceTintColor {
    pub const PIECE_TYPE_PERSONA_EYES: &'static str = "persona_eyes";
    pub const PIECE_TYPE_PERSONA_HAIR: &'static str = "persona_hair";
    pub const PIECE_TYPE_PERSONA_MOUTH: &'static str = "persona_mouth";

    pub fn new(piece_type: String, colors: Vec<String>) -> PersonaPieceTintColor {
        PersonaPieceTintColor { piece_type, colors }
    }

    pub fn piece_type(&self) -> String {
        self.piece_type.clone()
    }

    pub fn colors(&self) -> &Vec<String> {
        &self.colors
    }
}
