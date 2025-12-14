#[derive(serde::Serialize, Debug)]
pub struct PersonaSkinPiece {
    piece_id: String,
    piece_type: String,
    pack_id: String,
    is_default_piece: bool,
    product_id: String,
}

impl PersonaSkinPiece {
    pub const PIECE_TYPE_PERSONA_BODY: &'static str = "persona_body";
    pub const PIECE_TYPE_PERSONA_BOTTOM: &'static str = "persona_bottom";
    pub const PIECE_TYPE_PERSONA_EYES: &'static str = "persona_eyes";
    pub const PIECE_TYPE_PERSONA_FACIAL_HAIR: &'static str = "persona_facial_hair";
    pub const PIECE_TYPE_PERSONA_FEET: &'static str = "persona_feet";
    pub const PIECE_TYPE_PERSONA_HAIR: &'static str = "persona_hair";
    pub const PIECE_TYPE_PERSONA_MOUTH: &'static str = "persona_mouth";
    pub const PIECE_TYPE_PERSONA_SKELETON: &'static str = "persona_skeleton";
    pub const PIECE_TYPE_PERSONA_SKIN: &'static str = "persona_skin";
    pub const PIECE_TYPE_PERSONA_TOP: &'static str = "persona_top";

    pub fn new(
        piece_id: String,
        piece_type: String,
        pack_id: String,
        is_default_piece: bool,
        product_id: String,
    ) -> PersonaSkinPiece {
        PersonaSkinPiece {
            piece_id,
            piece_type,
            pack_id,
            is_default_piece,
            product_id,
        }
    }

    pub fn piece_id(&self) -> String {
        self.piece_id.clone()
    }

    pub fn piece_type(&self) -> String {
        self.piece_type.clone()
    }

    pub fn pack_id(&self) -> String {
        self.pack_id.clone()
    }

    pub fn is_default_piece(&self) -> bool {
        self.is_default_piece
    }

    pub fn product_id(&self) -> String {
        self.product_id.clone()
    }
}
