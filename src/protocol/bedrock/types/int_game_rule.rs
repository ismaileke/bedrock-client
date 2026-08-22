use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct IntGameRule {
    pub value: u32,
    pub is_player_modifiable: bool,
}

impl IntGameRule {
    pub fn is_player_modifiable(&self) -> bool {
        self.is_player_modifiable
    }

    pub fn new(value: u32, is_player_modifiable: bool) -> IntGameRule {
        IntGameRule {
            value,
            is_player_modifiable,
        }
    }

    pub fn read(stream: &mut Reader, is_player_modifiable: bool, _is_start_game: bool) -> IntGameRule {
        IntGameRule {
            value: stream.get_u32_le(),
            is_player_modifiable,
        }
    }

    pub fn write(&mut self, stream: &mut Writer, _is_start_game: bool) {
        stream.put_u32_le(self.value);
    }
}
