use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PhotoTransfer {
    pub photo_name: String,
    pub photo_data: String,
    pub book_id: String,
    pub photo_type: u8,
    pub source_type: u8,
    pub owner_actor_unique_id: i64,
    pub new_photo_name: String,
}

impl Packet for PhotoTransfer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPhotoTransfer.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.photo_name);
        PacketSerializer::put_string(stream, &self.photo_data);
        PacketSerializer::put_string(stream, &self.book_id);
        stream.put_u8(self.photo_type);
        stream.put_u8(self.source_type);
        stream.put_i64_le(self.owner_actor_unique_id);
        PacketSerializer::put_string(stream, &self.new_photo_name);
    }

    fn decode(stream: &mut Reader) -> PhotoTransfer {
        let photo_name = PacketSerializer::get_string(stream);
        let photo_data = PacketSerializer::get_string(stream);
        let book_id = PacketSerializer::get_string(stream);
        let photo_type = stream.get_u8();
        let source_type = stream.get_u8();
        let owner_actor_unique_id = stream.get_i64_le();
        let new_photo_name = PacketSerializer::get_string(stream);

        PhotoTransfer {
            photo_name,
            photo_data,
            book_id,
            photo_type,
            source_type,
            owner_actor_unique_id,
            new_photo_name,
        }
    }
}
