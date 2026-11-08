use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::play_sound::PlaySound;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundPlayAudioContent {
    pub shared_metadata: String,
    pub playback_content: String,
    pub playback_type: String,
    pub sound: PlaySound
}

impl Packet for ClientBoundPlayAudioContent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundPlayAudioContent.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.shared_metadata);
        PacketSerializer::put_string(stream, &self.playback_content);
        PacketSerializer::put_string(stream, &self.playback_type);
        self.sound.encode(stream);
    }

    fn decode(stream: &mut Reader) -> ClientBoundPlayAudioContent {
        let shared_metadata = PacketSerializer::get_string(stream);
        let playback_content = PacketSerializer::get_string(stream);
        let playback_type = PacketSerializer::get_string(stream);
        let sound = PlaySound::decode(stream);
        ClientBoundPlayAudioContent { shared_metadata, playback_content, playback_type, sound }
    }
}
