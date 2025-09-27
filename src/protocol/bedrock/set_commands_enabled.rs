use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct SetCommandsEnabled {
    pub enabled: bool
}

pub fn new(enabled: bool) -> SetCommandsEnabled {
    SetCommandsEnabled { enabled }
}

impl Packet for SetCommandsEnabled {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetCommandsEnabled.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_bool(self.enabled);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetCommandsEnabled {
        let mut stream = Stream::new(bytes, 0);

        let enabled = stream.get_bool();

        SetCommandsEnabled { enabled }
    }
    
    fn debug(&self) {
        println!("Enabled: {}", self.enabled);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
