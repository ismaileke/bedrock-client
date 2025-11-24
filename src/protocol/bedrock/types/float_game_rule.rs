use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct FloatGameRule {
    pub value: f32,
    pub is_player_modifiable: bool
}

impl FloatGameRule {

    pub fn is_player_modifiable(&self) -> bool {
        self.is_player_modifiable
    }

    pub fn new(value: f32, is_player_modifiable: bool) -> FloatGameRule {
        FloatGameRule{ value, is_player_modifiable }
    }

    pub fn read(stream: &mut Stream, is_player_modifiable: bool) -> FloatGameRule {
        FloatGameRule{ value: stream.get_f32_le(), is_player_modifiable }
    }

    pub fn write(&mut self, stream: &mut Stream, _is_start_game: bool) {
        stream.put_f32_le(self.value);
    }
}
