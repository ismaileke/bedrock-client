use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PauseSoundData {}

impl PauseSoundData {
    pub fn new() -> PauseSoundData { PauseSoundData {} }

    pub fn read(_stream: &mut Reader) -> PauseSoundData { PauseSoundData {} }

    pub fn write(&self, _stream: &mut Writer) {}
}
