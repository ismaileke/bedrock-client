use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundDataDrivenUIShowScreen {
    pub screen_id: String,
    pub form_id: u32,
    pub data_instance_id: Option<u32>
}

impl Packet for ClientBoundDataDrivenUIShowScreen {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundDataDrivenUIShowScreen.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.screen_id.clone());
        stream.put_u32_le(self.form_id);
        PacketSerializer::write_optional(&mut stream, &self.data_instance_id, |s, v| s.put_u32_le(*v));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ClientBoundDataDrivenUIShowScreen {
        let screen_id = PacketSerializer::get_string(stream);
        let form_id = stream.get_u32_le();
        let data_instance_id = PacketSerializer::read_optional(stream, |s| s.get_u32_le());

        ClientBoundDataDrivenUIShowScreen { screen_id, form_id, data_instance_id }
    }
}
