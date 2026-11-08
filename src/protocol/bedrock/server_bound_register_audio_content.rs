use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::audio::audio_content_registration_entry::AudioContentRegistrationEntry;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundRegisterAudioContent {
    pub registrations: Vec<AudioContentRegistrationEntry>
}

impl ServerBoundRegisterAudioContent {
    pub const AUDIO_CONTENT_PLAYBACK_TYPE_MUSIC: &'static str = "Music";
    pub const AUDIO_CONTENT_PLAYBACK_TYPE_SOUND: &'static str = "Sound";
}

impl Packet for ServerBoundRegisterAudioContent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundRegisterAudioContent.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.registrations.len() as u32);
        for registration in &self.registrations {
            registration.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> ServerBoundRegisterAudioContent {
        let count = stream.get_var_u32() as usize;
        let mut registrations = Vec::with_capacity(count);
        for _ in 0..count {
            registrations.push(AudioContentRegistrationEntry::read(stream));
        }
        ServerBoundRegisterAudioContent { registrations }
    }
}
