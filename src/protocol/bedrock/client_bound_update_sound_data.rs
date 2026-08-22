use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::sound::sound_data_update::SoundDataUpdate;

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundUpdateSoundData {
    pub server_sound_handle: u64,
    pub stop: Option<SoundDataUpdate>,
    pub set_volume: Option<SoundDataUpdate>,
    pub set_pitch: Option<SoundDataUpdate>,
    pub fade: Option<SoundDataUpdate>,
    pub seek_to: Option<SoundDataUpdate>,
    pub pause: Option<SoundDataUpdate>,
    pub resume: Option<SoundDataUpdate>,
}

impl Packet for ClientBoundUpdateSoundData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundUpdateSoundData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u64_le(self.server_sound_handle);
        PacketSerializer::write_optional(stream, &self.stop, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.set_volume, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.set_pitch, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.fade, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.seek_to, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.pause, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.resume, |s, v| v.write(s));
    }

    fn decode(stream: &mut Reader) -> ClientBoundUpdateSoundData {
        let server_sound_handle = stream.get_u64_le();
        let stop = PacketSerializer::read_optional(stream, |s| SoundDataUpdate::read(s));
        let set_volume = PacketSerializer::read_optional(stream, |s| SoundDataUpdate::read(s));
        let set_pitch = PacketSerializer::read_optional(stream, |s| SoundDataUpdate::read(s));
        let fade = PacketSerializer::read_optional(stream, |s| SoundDataUpdate::read(s));
        let seek_to = PacketSerializer::read_optional(stream, |s| SoundDataUpdate::read(s));
        let pause = PacketSerializer::read_optional(stream, |s| SoundDataUpdate::read(s));
        let resume = PacketSerializer::read_optional(stream, |s| SoundDataUpdate::read(s));

        ClientBoundUpdateSoundData { server_sound_handle, stop, set_volume, set_pitch, fade, seek_to, pause, resume }
    }
}
