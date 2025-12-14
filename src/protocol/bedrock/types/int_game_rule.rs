use binary_utils::binary::Stream;

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

    pub fn read(
        stream: &mut Stream,
        is_player_modifiable: bool,
        is_start_game: bool,
    ) -> IntGameRule {
        IntGameRule {
            value: if is_start_game {
                stream.get_var_u32()
            } else {
                stream.get_u32_le()
            },
            is_player_modifiable,
        }
    }

    pub fn write(&mut self, stream: &mut Stream, is_start_game: bool) {
        if is_start_game {
            stream.put_var_u32(self.value);
        } else {
            stream.put_u32_le(self.value);
        }
    }
}
