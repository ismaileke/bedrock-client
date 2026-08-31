#[derive(serde::Serialize, Debug)]
pub struct PersonaSkinPiece {
    pub piece_id: String,
    pub piece_type: i32,
    pub pack_id: String,
    pub is_default_piece: bool,
    pub product_id: String,
}

impl PersonaSkinPiece {
    pub const PIECE_TYPE_UNKNOWN: i32 = 0;
    pub const PIECE_TYPE_SKELETON: i32 = 1;
    pub const PIECE_TYPE_BODY: i32 = 2;
    pub const PIECE_TYPE_SKIN: i32 = 3;
    pub const PIECE_TYPE_BOTTOM: i32 = 4;
    pub const PIECE_TYPE_FEET: i32 = 5;
    pub const PIECE_TYPE_DRESS: i32 = 6;
    pub const PIECE_TYPE_TOP: i32 = 7;
    pub const PIECE_TYPE_HIGH_PANTS: i32 = 8;
    pub const PIECE_TYPE_HANDS: i32 = 9;
    pub const PIECE_TYPE_OUTERWEAR: i32 = 10;
    pub const PIECE_TYPE_FACIAL_HAIR: i32 = 11;
    pub const PIECE_TYPE_MOUTH: i32 = 12;
    pub const PIECE_TYPE_EYES: i32 = 13;
    pub const PIECE_TYPE_HAIR: i32 = 14;
    pub const PIECE_TYPE_HOOD: i32 = 15;
    pub const PIECE_TYPE_BACK: i32 = 16;
    pub const PIECE_TYPE_FACE_ACCESSORY: i32 = 17;
    pub const PIECE_TYPE_HEAD: i32 = 18;
    pub const PIECE_TYPE_LEGS: i32 = 19;
    pub const PIECE_TYPE_LEFT_LEG: i32 = 20;
    pub const PIECE_TYPE_RIGHT_LEG: i32 = 21;
    pub const PIECE_TYPE_ARMS: i32 = 22;
    pub const PIECE_TYPE_LEFT_ARM: i32 = 23;
    pub const PIECE_TYPE_RIGHT_ARM: i32 = 24;
    pub const PIECE_TYPE_CAPES: i32 = 25;
    pub const PIECE_TYPE_CLASSIC_SKIN: i32 = 26;
    pub const PIECE_TYPE_EMOTE: i32 = 27;
    pub const PIECE_TYPE_UNSUPPORTED: i32 = 28;

    pub fn new(
        piece_id: String,
        piece_type: i32,
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

    pub fn piece_id(&self) -> &String {
        &self.piece_id
    }

    pub fn piece_type(&self) -> i32 {
        self.piece_type
    }

    pub fn pack_id(&self) -> &String {
        &self.pack_id
    }

    pub fn is_default_piece(&self) -> bool {
        self.is_default_piece
    }

    pub fn product_id(&self) -> &String {
        &self.product_id
    }
}
