use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundUpdateSoundData {
    pub server_sound_handle: u64,
    pub sound_event: String,
}

impl Packet for ClientBoundUpdateSoundData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundUpdateSoundData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_u64_le(self.server_sound_handle);
        PacketSerializer::put_string(&mut stream, self.sound_event.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ClientBoundUpdateSoundData {
        let server_sound_handle = stream.get_u64_le();
        let sound_event = PacketSerializer::get_string(stream);

        ClientBoundUpdateSoundData { server_sound_handle, sound_event }
    }
}
