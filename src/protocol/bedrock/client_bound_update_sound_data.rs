use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::sound::sound_data_update::SoundDataUpdate;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundUpdateSoundData {
    pub server_sound_handle: u64,
    pub update: SoundDataUpdate
}

impl Packet for ClientBoundUpdateSoundData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundUpdateSoundData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u64_le(self.server_sound_handle);
        self.update.write(stream);
    }

    fn decode(stream: &mut Reader) -> ClientBoundUpdateSoundData {
        let server_sound_handle = stream.get_u64_le();
        let update = SoundDataUpdate::read(stream);

        ClientBoundUpdateSoundData { server_sound_handle, update }
    }
}
