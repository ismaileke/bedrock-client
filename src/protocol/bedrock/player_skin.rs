use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::skin::skin_data::SkinData;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct PlayerSkin {
    pub uuid: String,
    pub skin: SkinData,
    pub new_skin_name: String,
    pub old_skin_name: String,
}

pub fn new(
    uuid: String,
    skin: SkinData,
    new_skin_name: String,
    old_skin_name: String,
) -> PlayerSkin {
    PlayerSkin {
        uuid,
        skin,
        new_skin_name,
        old_skin_name,
    }
}

impl Packet for PlayerSkin {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerSkin.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_uuid(&mut stream, self.uuid.clone());
        PacketSerializer::put_skin(&mut stream, &self.skin);
        PacketSerializer::put_string(&mut stream, self.new_skin_name.clone());
        PacketSerializer::put_string(&mut stream, self.old_skin_name.clone());
        stream.put_bool(self.skin.is_verified);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PlayerSkin {
        let uuid = PacketSerializer::get_uuid(stream);
        let mut skin = PacketSerializer::get_skin(stream);
        let new_skin_name = PacketSerializer::get_string(stream);
        let old_skin_name = PacketSerializer::get_string(stream);
        skin.is_verified = stream.get_bool();

        PlayerSkin {
            uuid,
            skin,
            new_skin_name,
            old_skin_name,
        }
    }

    fn debug(&self) {
        println!("UUID: {}", self.uuid);
        println!("Skin: {:?}", self.skin);
        println!("New Skin: {:?}", self.new_skin_name);
        println!("Old Skin: {:?}", self.old_skin_name);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
