use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct SimpleEvent {
    pub event_type: u16
}

pub fn new(event_type: u16) -> SimpleEvent {
    SimpleEvent { event_type }
}

impl SimpleEvent {
    pub const TYPE_ENABLE_COMMANDS: u16 = 1;
    pub const TYPE_DISABLE_COMMANDS: u16 = 2;
    pub const TYPE_UNLOCK_WORLD_TEMPLATE_SETTINGS: u16 = 3;
}

impl Packet for SimpleEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSimpleEvent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_short(self.event_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SimpleEvent {
        let mut stream = Stream::new(bytes, 0);

        let event_type = stream.get_l_short();

        SimpleEvent { event_type }
    }

    fn debug(&self) {
        println!("Event Type: {}", self.event_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
