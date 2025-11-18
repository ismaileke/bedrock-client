use std::fmt::Debug;
use binary_utils::binary::Stream;

pub trait GameRule: Debug {
    fn id(&self) -> u32;
    fn is_player_modifiable(&self) -> bool;
    fn write(&mut self, stream: &mut Stream, is_start_game: bool);
}
