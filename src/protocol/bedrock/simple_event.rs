use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SimpleEvent {
    pub event_type: u16,
}

impl Packet for SimpleEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSimpleEvent.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u16_le(self.event_type);
    }

    fn decode(stream: &mut Reader) -> SimpleEvent {
        let event_type = stream.get_u16_le();

        SimpleEvent { event_type }
    }
}

impl SimpleEvent {
    pub const TYPE_ENABLE_COMMANDS: u16 = 1;
    pub const TYPE_DISABLE_COMMANDS: u16 = 2;
    pub const TYPE_UNLOCK_WORLD_TEMPLATE_SETTINGS: u16 = 3;
}
