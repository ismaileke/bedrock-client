use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ShowStoreOffer {
    pub offer_id: String,
    pub redirect_type: u8,
}

impl Packet for ShowStoreOffer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDShowStoreOffer.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_uuid(stream, self.offer_id.clone());
        stream.put_u8(self.redirect_type);
    }

    fn decode(stream: &mut Reader) -> ShowStoreOffer {
        let offer_id = PacketSerializer::get_uuid(stream);
        let redirect_type = stream.get_u8();

        ShowStoreOffer { offer_id, redirect_type }
    }
}

impl ShowStoreOffer {
    pub const MARKETPLACE: u8 = 0;
    pub const DRESSING_ROOM: u8 = 1;
    pub const THIRD_PARTY_SERVER_PAGE: u8 = 2;
}
