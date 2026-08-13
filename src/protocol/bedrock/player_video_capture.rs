use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerVideoCapture {
    pub is_recording: bool,
    pub frame_rate: Option<u32>,
    pub file_prefix: Option<String>,
}

impl Packet for PlayerVideoCapture {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerVideoCapture.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_bool(self.is_recording);
        if self.is_recording {
            if self.frame_rate.is_none() {
                panic!("PlayerUpdateEntityOverridesPacket with recording=true require a frame rate to be provided")
            }
            if self.file_prefix.is_none() {
                panic!("PlayerUpdateEntityOverridesPacket with recording=true require a file prefix to be provided")
            }
        }
        stream.put_u32_le(self.frame_rate.unwrap());
        PacketSerializer::put_string(stream, self.file_prefix.clone().unwrap());
    }

    fn decode(stream: &mut Reader) -> PlayerVideoCapture {
        let is_recording = stream.get_bool();
        let mut frame_rate = None;
        let mut file_prefix = None;
        if is_recording {
            frame_rate = Some(stream.get_u32_le());
            file_prefix = Some(PacketSerializer::get_string(stream));
        }

        PlayerVideoCapture { is_recording, frame_rate, file_prefix }
    }
}
