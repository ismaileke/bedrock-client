use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlaySound {
    pub sound_name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub volume: f32,
    pub pitch: f32,
    pub server_sound_handle: Option<u64>,
}

impl Packet for PlaySound {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlaySound.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.sound_name);
        PacketSerializer::put_block_pos(
            stream,
            &vec![
                (self.x * 8.0) as i32,
                (self.y * 8.0) as i32,
                (self.z * 8.0) as i32,
            ],
        );
        stream.put_f32_le(self.volume);
        stream.put_f32_le(self.pitch);
        PacketSerializer::write_optional(stream, &self.server_sound_handle, |s, v| s.put_u64_le(*v));
    }

    fn decode(stream: &mut Reader) -> PlaySound {
        let sound_name = PacketSerializer::get_string(stream);
        let block_pos = PacketSerializer::get_block_pos(stream);
        let volume = stream.get_f32_le();
        let pitch = stream.get_f32_le();
        let x = (block_pos[0] as f32) / 8.0;
        let y = (block_pos[1] as f32) / 8.0;
        let z = (block_pos[2] as f32) / 8.0;
        let server_sound_handle = PacketSerializer::read_optional(stream, |s| s.get_u64_le());

        PlaySound { sound_name, x, y, z, volume, pitch, server_sound_handle }
    }
}
