use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CreatePhoto {
    pub actor_unique_id: i64,
    pub photo_name: String,
    pub photo_item_name: String,
}

impl Packet for CreatePhoto {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCreatePhoto.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_i64_le(self.actor_unique_id);
        PacketSerializer::put_string(stream, &self.photo_name);
        PacketSerializer::put_string(stream, &self.photo_item_name);
    }

    fn decode(stream: &mut Reader) -> CreatePhoto {
        let actor_unique_id = stream.get_i64_le();
        let photo_name = PacketSerializer::get_string(stream);
        let photo_item_name = PacketSerializer::get_string(stream);

        CreatePhoto { actor_unique_id, photo_name, photo_item_name }
    }
}
