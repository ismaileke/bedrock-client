use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ContainerClose {
    pub window_id: u8,
    pub window_type: u8,
    pub server: bool,
}

impl Packet for ContainerClose {
    fn id(&self) -> u16 {
        BedrockPacketType::IDContainerClose.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.window_id);
        stream.put_u8(self.window_type);
        stream.put_bool(self.server);
    }

    fn decode(stream: &mut Reader) -> ContainerClose {
        let window_id = stream.get_u8();
        let window_type = stream.get_u8();
        let server = stream.get_bool();

        ContainerClose { window_id, window_type, server }
    }
}
