use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ResourcePackClientResponse {
    pub status: u8,
    pub pack_ids: Vec<String>,
}

impl Packet for ResourcePackClientResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackClientResponse.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.status);
        stream.put_u16_le(self.pack_ids.len() as u16);

        for pack_id in &self.pack_ids {
            PacketSerializer::put_string(stream, pack_id.clone());
        }
    }

    fn decode(stream: &mut Reader) -> ResourcePackClientResponse {
        let status = stream.get_u8();
        let entry_count = stream.get_u16_le();

        let mut pack_ids = vec![];
        for _ in 0..entry_count {
            let pack_id = PacketSerializer::get_string(stream);
            pack_ids.push(pack_id);
        }

        ResourcePackClientResponse { status, pack_ids }
    }
}

impl ResourcePackClientResponse {
    pub const NONE: u8 = 0;
    pub const REFUSED: u8 = 1;
    pub const SEND_PACKS: u8 = 2;
    pub const HAVE_ALL_PACKS: u8 = 3;
    pub const COMPLETED: u8 = 4;
}
