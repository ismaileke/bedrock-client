use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::presence_config::PresenceConfig;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ServerPresenceInfo {
    pub presence_config: Option<PresenceConfig>,
}

impl Packet for ServerPresenceInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerPresenceInfo.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::write_optional(&mut stream, &self.presence_config, |s, v| v.write(s));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ServerPresenceInfo {
        let presence_config = PacketSerializer::read_optional(stream, |s| PresenceConfig::read(s));
        ServerPresenceInfo { presence_config }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
