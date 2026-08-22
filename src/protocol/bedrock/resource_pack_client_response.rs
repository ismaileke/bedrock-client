use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ResourcePackClientResponse {
    pub status: u32,
    pub pack_ids: Vec<String>
}

impl Packet for ResourcePackClientResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackClientResponse.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.status);
        let name = ResourcePackClientResponse::RESPONSE_STATUS[self.status as usize];
        PacketSerializer::put_string(stream, name);
        if self.status == ResourcePackClientResponse::SEND_PACKS {
            stream.put_var_u32(self.pack_ids.len() as u32);
            for pack_id in &self.pack_ids {
                PacketSerializer::put_string(stream, &pack_id);
            }
        }
    }

    fn decode(stream: &mut Reader) -> ResourcePackClientResponse {
        let status = stream.get_var_u32();
        let _ = PacketSerializer::get_string(stream);
        let mut pack_ids = vec![];
        if status == ResourcePackClientResponse::SEND_PACKS {
            let entry_count = stream.get_var_u32();
            for _ in 0..entry_count {
                pack_ids.push(PacketSerializer::get_string(stream));
            }
        }
        ResourcePackClientResponse { status, pack_ids }
    }
}

impl ResourcePackClientResponse {
    pub const REFUSED: u32 = 0;
    pub const SEND_PACKS: u32 = 1;
    pub const HAVE_ALL_PACKS: u32 = 2;
    pub const COMPLETED: u32 = 3;

    pub const RESPONSE_STATUS: [&str; 4] = ["cancel", "downloading", "downloadingfinished", "resourcepackstackfinished"];
}
