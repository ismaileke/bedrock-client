use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ResumeSoundData {}

impl ResumeSoundData {
    pub fn new() -> ResumeSoundData { ResumeSoundData {} }

    pub fn read(_stream: &mut Reader) -> ResumeSoundData { ResumeSoundData {} }

    pub fn write(&self, _stream: &mut Writer) {}
}
