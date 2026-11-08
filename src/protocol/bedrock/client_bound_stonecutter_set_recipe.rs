use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundStonecutterSetRecipe {
    pub actor_unique_id: i64,
    pub container_id: u8,
    pub recipe_index: i32
}

impl Packet for ClientBoundStonecutterSetRecipe {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundStonecutterSetRecipe.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        stream.put_u8(self.container_id);
        stream.put_var_i32(self.recipe_index);
    }

    fn decode(stream: &mut Reader) -> ClientBoundStonecutterSetRecipe {
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let container_id = stream.get_u8();
        let recipe_index = stream.get_var_i32();
        ClientBoundStonecutterSetRecipe { actor_unique_id, container_id, recipe_index }
    }
}
