use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::sound::sound_data_update::SoundDataUpdate;

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundUpdateSoundData {
    pub server_sound_handle: u64,
    pub stop: SoundDataUpdate,
    pub set_volume: SoundDataUpdate,
    pub set_pitch: SoundDataUpdate,
    pub fade: SoundDataUpdate,
    pub seek_to: SoundDataUpdate,
    pub pause: SoundDataUpdate,
    pub resume: SoundDataUpdate,
}

impl Packet for ClientBoundUpdateSoundData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundUpdateSoundData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u64_le(self.server_sound_handle);
        self.stop.write(stream);
        self.set_volume.write(stream);
        self.set_pitch.write(stream);
        self.fade.write(stream);
        self.seek_to.write(stream);
        self.pause.write(stream);
        self.resume.write(stream);
    }

    fn decode(stream: &mut Reader) -> ClientBoundUpdateSoundData {
        let server_sound_handle = stream.get_u64_le();
        let stop = SoundDataUpdate::read(stream);
        let set_volume = SoundDataUpdate::read(stream);
        let set_pitch = SoundDataUpdate::read(stream);
        let fade = SoundDataUpdate::read(stream);
        let seek_to = SoundDataUpdate::read(stream);
        let pause = SoundDataUpdate::read(stream);
        let resume = SoundDataUpdate::read(stream);

        ClientBoundUpdateSoundData { server_sound_handle, stop, set_volume, set_pitch, fade, seek_to, pause, resume }
    }
}
