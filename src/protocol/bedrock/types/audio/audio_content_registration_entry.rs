use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::audio::signed_audio_content::SignedAudioContent;

#[derive(serde::Serialize, Debug)]
pub struct AudioContentRegistrationEntry {
    pub audio_content_id: String,
    pub shared_metadata: SignedAudioContent,
    pub server_content: SignedAudioContent,
    pub playback_content: SignedAudioContent
}

impl AudioContentRegistrationEntry {
    pub fn new(audio_content_id: String, shared_metadata: SignedAudioContent, server_content: SignedAudioContent, playback_content: SignedAudioContent) -> AudioContentRegistrationEntry {
        AudioContentRegistrationEntry { audio_content_id, shared_metadata, server_content, playback_content }
    }

    pub fn read(stream: &mut Reader) -> AudioContentRegistrationEntry {
        let audio_content_id = PacketSerializer::get_string(stream);
        let shared_metadata = SignedAudioContent::read(stream);
        let server_content = SignedAudioContent::read(stream);
        let playback_content = SignedAudioContent::read(stream);

        AudioContentRegistrationEntry { audio_content_id, shared_metadata, server_content, playback_content }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.audio_content_id);
        self.shared_metadata.write(stream);
        self.server_content.write(stream);
        self.playback_content.write(stream);
    }
}
