use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct PlayerVideoCapture {
    pub is_recording: bool,
    pub frame_rate: Option<u32>,
    pub file_prefix: Option<String>,
}

pub fn new(
    is_recording: bool,
    frame_rate: Option<u32>,
    file_prefix: Option<String>,
) -> PlayerVideoCapture {
    PlayerVideoCapture {
        is_recording,
        frame_rate,
        file_prefix,
    }
}

impl Packet for PlayerVideoCapture {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerVideoCapture.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

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
        PacketSerializer::put_string(&mut stream, self.file_prefix.clone().unwrap());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PlayerVideoCapture {
        let is_recording = stream.get_bool();
        let mut frame_rate = None;
        let mut file_prefix = None;
        if is_recording {
            frame_rate = Some(stream.get_u32_le());
            file_prefix = Some(PacketSerializer::get_string(stream));
        }

        PlayerVideoCapture {
            is_recording,
            frame_rate,
            file_prefix,
        }
    }

    fn debug(&self) {
        println!("Is Recording: {}", self.is_recording);
        println!("Frame Rate: {:?}", self.frame_rate);
        println!("File Prefix: {:?}", self.file_prefix);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
