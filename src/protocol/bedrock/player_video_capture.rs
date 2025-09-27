use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct PlayerVideoCapture {
    pub is_recording: bool,
    pub frame_rate: Option<u32>,
    pub file_prefix: Option<String>
}

pub fn new(is_recording: bool, frame_rate: Option<u32>, file_prefix: Option<String>) -> PlayerVideoCapture {
    PlayerVideoCapture { is_recording, frame_rate, file_prefix }
}

impl Packet for PlayerVideoCapture {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerVideoCapture.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_bool(self.is_recording);
        if self.is_recording {
            if self.frame_rate.is_none() {
                panic!("PlayerUpdateEntityOverridesPacket with recording=true require a frame rate to be provided")
            }
            if self.file_prefix.is_none() {
                panic!("PlayerUpdateEntityOverridesPacket with recording=true require a file prefix to be provided")
            }
        }
        stream.put_l_int(self.frame_rate.unwrap());
        PacketSerializer::put_string(&mut stream, self.file_prefix.clone().unwrap());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PlayerVideoCapture {
        let mut stream = Stream::new(bytes, 0);

        let is_recording = stream.get_bool();
        let mut frame_rate = None;
        let mut file_prefix = None;
        if is_recording {
            frame_rate = Some(stream.get_l_int());
            file_prefix = Some(PacketSerializer::get_string(&mut stream));
        }

        PlayerVideoCapture { is_recording, frame_rate, file_prefix }
    }

    fn debug(&self) {
        println!("Is Recording: {}", self.is_recording);
        println!("Frame Rate: {:?}", self.frame_rate);
        println!("File Prefix: {:?}", self.file_prefix);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
