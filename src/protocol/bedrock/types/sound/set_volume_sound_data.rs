use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetVolumeSoundData {
    pub volume: f32
}

impl SetVolumeSoundData {
    pub fn new(volume: f32) -> SetVolumeSoundData {
        SetVolumeSoundData { volume }
    }

    pub fn read(stream: &mut Reader) -> SetVolumeSoundData {
        let volume = stream.get_f32_le();
        SetVolumeSoundData { volume }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_f32_le(self.volume);
    }
}
