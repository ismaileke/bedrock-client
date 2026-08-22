use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct StopSoundData {}

impl StopSoundData {
    pub fn new() -> StopSoundData { StopSoundData {} }

    pub fn read(_stream: &mut Reader) -> StopSoundData { StopSoundData {} }

    pub fn write(&self, _stream: &mut Writer) {}
}
