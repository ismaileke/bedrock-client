use binary_utils::binary::{Reader, Writer};
use std::fmt::Debug;
use crate::protocol::bedrock::types::sound::fade_sound_data::FadeSoundData;
use crate::protocol::bedrock::types::sound::pause_sound_data::PauseSoundData;
use crate::protocol::bedrock::types::sound::resume_sound_data::ResumeSoundData;
use crate::protocol::bedrock::types::sound::seek_to_sound_data::SeekToSoundData;
use crate::protocol::bedrock::types::sound::set_pitch_sound_data::SetPitchSoundData;
use crate::protocol::bedrock::types::sound::set_volume_sound_data::SetVolumeSoundData;
use crate::protocol::bedrock::types::sound::stop_sound_data::StopSoundData;

#[derive(serde::Serialize, Debug)]
pub enum SoundDataUpdate {
    Stop(StopSoundData),
    SetVolume(SetVolumeSoundData),
    SetPitch(SetPitchSoundData),
    Fade(FadeSoundData),
    SeekTo(SeekToSoundData),
    Pause(PauseSoundData),
    Resume(ResumeSoundData),
}

impl SoundDataUpdate {
    pub const STOP: u32 = 0;
    pub const SET_VOLUME: u32 = 1;
    pub const SET_PITCH: u32 = 2;
    pub const FADE: u32 = 3;
    pub const SEEK_TO: u32 = 4;
    pub const PAUSE: u32 = 5;
    pub const RESUME: u32 = 6;

    pub fn id(&self) -> u32 {
        match self {
            SoundDataUpdate::Stop(_) => Self::STOP,
            SoundDataUpdate::SetVolume(_) => Self::SET_VOLUME,
            SoundDataUpdate::SetPitch(_) => Self::SET_PITCH,
            SoundDataUpdate::Fade(_) => Self::FADE,
            SoundDataUpdate::SeekTo(_) => Self::SEEK_TO,
            SoundDataUpdate::Pause(_) => Self::PAUSE,
            SoundDataUpdate::Resume(_) => Self::RESUME,
        }
    }

    pub fn read(stream: &mut Reader) -> SoundDataUpdate {
        let sound_type = stream.get_var_u32();
        match sound_type {
            SoundDataUpdate::STOP => SoundDataUpdate::Stop(StopSoundData::read(stream)),
            SoundDataUpdate::SET_VOLUME => SoundDataUpdate::SetVolume(SetVolumeSoundData::read(stream)),
            SoundDataUpdate::SET_PITCH => SoundDataUpdate::SetPitch(SetPitchSoundData::read(stream)),
            SoundDataUpdate::FADE => SoundDataUpdate::Fade(FadeSoundData::read(stream)),
            SoundDataUpdate::SEEK_TO => SoundDataUpdate::SeekTo(SeekToSoundData::read(stream)),
            SoundDataUpdate::PAUSE => SoundDataUpdate::Pause(PauseSoundData::read(stream)),
            SoundDataUpdate::RESUME => SoundDataUpdate::Resume(ResumeSoundData::read(stream)),
            _ => panic!("Sound type not handled: {}", sound_type),
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        match self {
            SoundDataUpdate::Stop(r) => r.write(stream),
            SoundDataUpdate::SetVolume(r) => r.write(stream),
            SoundDataUpdate::SetPitch(r) => r.write(stream),
            SoundDataUpdate::Fade(r) => r.write(stream),
            SoundDataUpdate::SeekTo(r) => r.write(stream),
            SoundDataUpdate::Pause(r) => r.write(stream),
            SoundDataUpdate::Resume(r) => r.write(stream),
        }
    }
}
