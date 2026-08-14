use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SpawnExperienceOrb {
    pub position: Vec<f32>,
    pub amount: i32,
}

impl Packet for SpawnExperienceOrb {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSpawnExperienceOrb.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_vector3(stream, &self.position);
        stream.put_var_i32(self.amount);
    }

    fn decode(stream: &mut Reader) -> SpawnExperienceOrb {
        let position = PacketSerializer::get_vector3(stream);
        let amount = stream.get_var_i32();

        SpawnExperienceOrb { position, amount }
    }
}
