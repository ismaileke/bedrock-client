use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PurchaseReceipt {
    pub entries: Vec<String>,
}

impl Packet for PurchaseReceipt {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPurchaseReceipt.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.entries.len() as u32);
        for entry in self.entries.iter() {
            PacketSerializer::put_string(stream, entry.to_string());
        }
    }

    fn decode(stream: &mut Reader) -> PurchaseReceipt {
        let mut entries = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            entries.push(PacketSerializer::get_string(stream));
        }

        PurchaseReceipt { entries }
    }
}
