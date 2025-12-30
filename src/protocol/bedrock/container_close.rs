use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ContainerClose {
    pub window_id: u8,
    pub window_type: u8,
    pub server: bool,
}

impl Packet for ContainerClose {
    fn id(&self) -> u16 {
        BedrockPacketType::IDContainerClose.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.window_id);
        stream.put_byte(self.window_type);
        stream.put_bool(self.server);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ContainerClose {
        let window_id = stream.get_byte();
        let window_type = stream.get_byte();
        let server = stream.get_bool();

        ContainerClose { window_id, window_type, server }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
