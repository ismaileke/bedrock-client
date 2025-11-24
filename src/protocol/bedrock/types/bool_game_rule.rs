use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BoolGameRule {
    pub value: bool,
    pub is_player_modifiable: bool
}

impl BoolGameRule {
    pub fn is_player_modifiable(&self) -> bool {
        self.is_player_modifiable
    }

    pub fn new(value: bool, is_player_modifiable: bool) -> BoolGameRule {
        BoolGameRule{ value, is_player_modifiable }
    }

    pub fn read(stream: &mut Stream, is_player_modifiable: bool) -> BoolGameRule {
        BoolGameRule{ value: stream.get_bool(), is_player_modifiable }
    }

    pub fn write(&mut self, stream: &mut Stream, _is_start_game: bool) {
        stream.put_bool(self.value);
    }
}
