use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct StopSound {
    pub sound_name: String,
    pub stop_all: bool,
    pub stop_legacy_music: bool,
}

impl Packet for StopSound {
    fn id(&self) -> u16 {
        BedrockPacketType::IDStopSound.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.sound_name);
        stream.put_bool(self.stop_all);
        stream.put_bool(self.stop_legacy_music);
    }

    fn decode(stream: &mut Reader) -> StopSound {
        let sound_name = PacketSerializer::get_string(stream);
        let stop_all = stream.get_bool();
        let stop_legacy_music = stream.get_bool();

        StopSound {
            sound_name,
            stop_all,
            stop_legacy_music,
        }
    }
}
