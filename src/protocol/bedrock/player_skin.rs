use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::skin::skin_data::SkinData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerSkin {
    pub uuid: String,
    pub skin: SkinData,
    pub old_skin_name: String,
    pub new_skin_name: String
}

impl Packet for PlayerSkin {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerSkin.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_uuid(stream, &self.uuid);
        PacketSerializer::put_skin(stream, &self.skin);
        PacketSerializer::put_string(stream, &self.old_skin_name);
        PacketSerializer::put_string(stream, &self.new_skin_name);
    }

    fn decode(stream: &mut Reader) -> PlayerSkin {
        let uuid = PacketSerializer::get_uuid(stream);
        let skin = PacketSerializer::get_skin(stream);
        let old_skin_name = PacketSerializer::get_string(stream);
        let new_skin_name = PacketSerializer::get_string(stream);

        PlayerSkin { uuid, skin, new_skin_name, old_skin_name }
    }
}
