use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct PurchaseReceipt {
    pub entries: Vec<String>
}

pub fn new(entries: Vec<String>) -> PurchaseReceipt {
    PurchaseReceipt { entries }
}

impl Packet for PurchaseReceipt {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPurchaseReceipt.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.entries.len() as u32);
        for entry in self.entries.iter() {
            PacketSerializer::put_string(&mut stream, entry.to_string());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PurchaseReceipt {
        let mut entries = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            entries.push(PacketSerializer::get_string(stream));
        }

        PurchaseReceipt { entries }
    }
    
    fn debug(&self) {
        println!("Entries: {:?}", self.entries);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
