use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetPitchSoundData {
    pub pitch: f32
}

impl SetPitchSoundData {
    pub fn new(pitch: f32) -> SetPitchSoundData {
        SetPitchSoundData { pitch }
    }

    pub fn read(stream: &mut Reader) -> SetPitchSoundData {
        let pitch = stream.get_f32_le();
        SetPitchSoundData { pitch }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_f32_le(self.pitch);
    }
}
