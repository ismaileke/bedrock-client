use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SeekToSoundData {
    pub seconds: f32
}

impl SeekToSoundData {
    pub fn new(seconds: f32) -> SeekToSoundData {
        SeekToSoundData { seconds }
    }

    pub fn read(stream: &mut Reader) -> SeekToSoundData {
        let seconds = stream.get_f32_le();
        SeekToSoundData { seconds }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_f32_le(self.seconds);
    }
}
