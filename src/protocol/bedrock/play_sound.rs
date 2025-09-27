use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct PlaySound {
    pub sound_name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub volume: f32,
    pub pitch: f32
}

pub fn new(sound_name: String, x: f32, y: f32, z: f32, volume: f32, pitch: f32) -> PlaySound {
    PlaySound { sound_name, x, y, z, volume, pitch }
}

impl Packet for PlaySound {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlaySound.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.sound_name.clone());
        PacketSerializer::put_block_pos(&mut stream, vec![(self.x * 8.0) as i32, (self.y * 8.0) as i32, (self.z * 8.0) as i32]);
        stream.put_l_float(self.volume);
        stream.put_l_float(self.pitch);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PlaySound {
        let mut stream = Stream::new(bytes, 0);

        let sound_name = PacketSerializer::get_string(&mut stream);
        let block_pos = PacketSerializer::get_block_pos(&mut stream);
        let volume = stream.get_l_float();
        let pitch = stream.get_l_float();
        let x = (block_pos[0] as f32) / 8.0;
        let y = (block_pos[1] as f32) / 8.0;
        let z = (block_pos[2] as f32) / 8.0;

        PlaySound { sound_name, x, y, z, volume, pitch }
    }

    fn debug(&self) {
        println!("Sound Name: {}", self.sound_name);
        println!("X: {}", self.x);
        println!("Y: {}", self.y);
        println!("Z: {}", self.z);
        println!("Volume: {}", self.volume);
        println!("Pitch: {}", self.pitch);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
