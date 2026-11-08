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
    pub const STOP: u8 = 0;
    pub const SET_VOLUME: u8 = 1;
    pub const SET_PITCH: u8 = 2;
    pub const FADE: u8 = 3;
    pub const SEEK_TO: u8 = 4;
    pub const PAUSE: u8 = 5;
    pub const RESUME: u8 = 6;

    pub fn id(&self) -> u8 {
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
        let variant = stream.get_var_u32();
        let event = stream.get_u8();
        if variant != event as u32 {
            panic!("Sound data variant index {} does not match event type {}", variant, event);
        }
        match event {
            SoundDataUpdate::STOP => SoundDataUpdate::Stop(StopSoundData::read(stream)),
            SoundDataUpdate::SET_VOLUME => SoundDataUpdate::SetVolume(SetVolumeSoundData::read(stream)),
            SoundDataUpdate::SET_PITCH => SoundDataUpdate::SetPitch(SetPitchSoundData::read(stream)),
            SoundDataUpdate::FADE => SoundDataUpdate::Fade(FadeSoundData::read(stream)),
            SoundDataUpdate::SEEK_TO => SoundDataUpdate::SeekTo(SeekToSoundData::read(stream)),
            SoundDataUpdate::PAUSE => SoundDataUpdate::Pause(PauseSoundData::read(stream)),
            SoundDataUpdate::RESUME => SoundDataUpdate::Resume(ResumeSoundData::read(stream)),
            _ => panic!("Sound type not handled: {}", event),
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.id() as u32);
        stream.put_u8(self.id());
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
