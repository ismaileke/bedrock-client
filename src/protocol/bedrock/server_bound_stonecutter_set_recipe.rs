use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundStonecutterSetRecipe {
    pub container_id: u8,
    pub recipe_index: i32
}

impl Packet for ServerBoundStonecutterSetRecipe {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundStonecutterSetRecipe.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.container_id);
        stream.put_var_i32(self.recipe_index);
    }

    fn decode(stream: &mut Reader) -> ServerBoundStonecutterSetRecipe {
        let container_id = stream.get_u8();
        let recipe_index = stream.get_var_i32();
        ServerBoundStonecutterSetRecipe { container_id, recipe_index }
    }
}
