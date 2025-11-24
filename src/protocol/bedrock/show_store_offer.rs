use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct ShowStoreOffer {
    pub offer_id: String,
    pub redirect_type: u8
}

pub fn new(offer_id: String, redirect_type: u8) -> ShowStoreOffer {
    ShowStoreOffer { offer_id, redirect_type }
}

impl Packet for ShowStoreOffer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDShowStoreOffer.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_uuid(&mut stream, self.offer_id.clone());
        stream.put_byte(self.redirect_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ShowStoreOffer {
        let offer_id = PacketSerializer::get_uuid(stream);
        let redirect_type = stream.get_byte();

        ShowStoreOffer { offer_id, redirect_type }
    }
    
    fn debug(&self) {
        println!("Offer ID: {}", self.offer_id);
        println!("Redirect Type: {}", self.redirect_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl ShowStoreOffer {
    pub const MARKETPLACE: u8 = 0;
    pub const DRESSING_ROOM: u8 = 1;
    pub const THIRD_PARTY_SERVER_PAGE: u8 = 2;
}
