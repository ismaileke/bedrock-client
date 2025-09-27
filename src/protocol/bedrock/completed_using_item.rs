use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct CompletedUsingItem {
    pub item_id: u16,
    pub action: u32
}

pub fn new(item_id: u16, action: u32) -> CompletedUsingItem {
    CompletedUsingItem { item_id, action }
}

impl Packet for CompletedUsingItem {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCompletedUsingItem.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_short(self.item_id);
        stream.put_l_int(self.action);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CompletedUsingItem {
        let mut stream = Stream::new(bytes, 0);

        let item_id = stream.get_short();
        let action = stream.get_l_int();

        CompletedUsingItem { item_id, action }
    }

    fn debug(&self) {
        println!("Item ID: {}", self.item_id);
        println!("Action: {}", self.action);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
