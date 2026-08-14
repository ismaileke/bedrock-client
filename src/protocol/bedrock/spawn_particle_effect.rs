use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SpawnParticleEffect {
    pub dimension_id: u8,
    pub actor_unique_id: i64,
    pub position: Vec<f32>,
    pub particle_name: String,
    pub molang_variables_json: Option<String>,
}

impl Packet for SpawnParticleEffect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSpawnParticleEffect.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.dimension_id);
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        PacketSerializer::put_vector3(stream, &self.position);
        PacketSerializer::put_string(stream, &self.particle_name);
        PacketSerializer::write_optional(stream, &self.molang_variables_json, |s, v| PacketSerializer::put_string(s, v));
    }

    fn decode(stream: &mut Reader) -> SpawnParticleEffect {
        let dimension_id = stream.get_u8();
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let position = PacketSerializer::get_vector3(stream);
        let particle_name = PacketSerializer::get_string(stream);
        let molang_variables_json = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));

        SpawnParticleEffect {
            dimension_id,
            actor_unique_id,
            position,
            particle_name,
            molang_variables_json,
        }
    }
}
