use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct FadeSoundData {
    pub duration: f32,
    pub target_volume: f32
}

impl FadeSoundData {
    pub fn new(duration: f32, target_volume: f32) -> FadeSoundData {
        FadeSoundData { duration, target_volume }
    }

    pub fn read(stream: &mut Reader) -> FadeSoundData {
        let duration = stream.get_f32_le();
        let target_volume = stream.get_f32_le();
        FadeSoundData { duration, target_volume }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_f32_le(self.duration);
        stream.put_f32_le(self.target_volume);
    }
}
