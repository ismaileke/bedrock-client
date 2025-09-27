use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct TickingAreasLoadStatus {
    pub waiting_for_preload: bool
}

pub fn new(waiting_for_preload: bool) -> TickingAreasLoadStatus {
    TickingAreasLoadStatus { waiting_for_preload }
}

impl Packet for TickingAreasLoadStatus {
    fn id(&self) -> u16 {
        BedrockPacketType::IDTickingAreasLoadStatus.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_bool(self.waiting_for_preload);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> TickingAreasLoadStatus {
        let mut stream = Stream::new(bytes, 0);

        let waiting_for_preload = stream.get_bool();

        TickingAreasLoadStatus { waiting_for_preload }
    }

    fn debug(&self) {
        println!("Waiting For Preload: {}", self.waiting_for_preload);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
