use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct StopSound {
    pub sound_name: String,
    pub stop_all: bool,
    pub stop_legacy_music: bool
}

pub fn new(sound_name: String, stop_all: bool, stop_legacy_music: bool) -> StopSound {
    StopSound { sound_name, stop_all, stop_legacy_music }
}

impl Packet for StopSound {
    fn id(&self) -> u16 {
        BedrockPacketType::IDStopSound.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.sound_name.clone());
        stream.put_bool(self.stop_all);
        stream.put_bool(self.stop_legacy_music);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> StopSound {
        let mut stream = Stream::new(bytes, 0);

        let sound_name = PacketSerializer::get_string(&mut stream);
        let stop_all = stream.get_bool();
        let stop_legacy_music = stream.get_bool();

        StopSound { sound_name, stop_all, stop_legacy_music }
    }

    fn debug(&self) {
        println!("Sound Name: {}", self.sound_name);
        println!("Stop All: {}", self.stop_all);
        println!("Stop Legacy Music: {}", self.stop_legacy_music);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
