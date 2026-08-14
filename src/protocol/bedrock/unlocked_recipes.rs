use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct UnlockedRecipes {
    pub unlock_type: u32,
    pub recipes: Vec<String>,
}

impl Packet for UnlockedRecipes {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUnlockedRecipes.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u32_le(self.unlock_type);
        stream.put_var_u32(self.recipes.len() as u32);
        for recipe in self.recipes.iter() {
            PacketSerializer::put_string(stream, recipe);
        }
    }

    fn decode(stream: &mut Reader) -> UnlockedRecipes {
        let unlock_type = stream.get_u32_le();
        let recipes_len = stream.get_var_u32() as usize;
        let mut recipes = Vec::new();
        for _ in 0..recipes_len {
            let recipe = PacketSerializer::get_string(stream);
            recipes.push(recipe);
        }

        UnlockedRecipes { unlock_type, recipes }
    }
}

impl UnlockedRecipes {
    pub const TYPE_EMPTY: u32 = 0;
    pub const TYPE_INITIALLY_UNLOCKED: u32 = 1;
    pub const TYPE_NEWLY_UNLOCKED: u32 = 2;
    pub const TYPE_REMOVE: u32 = 3;
    pub const TYPE_REMOVE_ALL: u32 = 4;
}
