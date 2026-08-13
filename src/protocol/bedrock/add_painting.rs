use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AddPainting {
    pub actor_unique_id: i64,
    pub actor_runtime_id: u64,
    pub position: Vec<f32>,
    pub direction: i32,
    pub title: String,
}

impl Packet for AddPainting {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddPainting.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        PacketSerializer::put_vector3(stream, self.position.clone());
        stream.put_var_i32(self.direction);
        PacketSerializer::put_string(stream, self.title.clone());
    }

    fn decode(stream: &mut Reader) -> AddPainting {
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let position = PacketSerializer::get_vector3(stream);
        let direction = stream.get_var_i32();
        let title = PacketSerializer::get_string(stream);

        AddPainting {
            actor_unique_id,
            actor_runtime_id,
            position,
            direction,
            title,
        }
    }
}
