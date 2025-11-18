use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct UnlockedRecipes {
    pub unlock_type: u32,
    pub recipes: Vec<String>
}

pub fn new(unlock_type: u32, recipes: Vec<String>) -> UnlockedRecipes {
    UnlockedRecipes { unlock_type, recipes }
}

impl Packet for UnlockedRecipes {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUnlockedRecipes.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_u32_le(self.unlock_type);
        stream.put_var_u32(self.recipes.len() as u32);
        for recipe in self.recipes.iter() {
            PacketSerializer::put_string(&mut stream, recipe.clone());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> UnlockedRecipes {
        let mut stream = Stream::new(bytes, 0);

        let unlock_type = stream.get_u32_le();
        let recipes_len = stream.get_var_u32() as usize;
        let mut recipes = Vec::new();
        for _ in 0..recipes_len {
            let recipe = PacketSerializer::get_string(&mut stream);
            recipes.push(recipe);
        }

        UnlockedRecipes { unlock_type, recipes }
    }

    fn debug(&self) {
        println!("Unlock Type: {}", self.unlock_type);
        println!("Recipes: {:?}", self.recipes);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl UnlockedRecipes {
    pub const TYPE_EMPTY: u32 = 0;
    pub const TYPE_INITIALLY_UNLOCKED: u32 = 1;
    pub const TYPE_NEWLY_UNLOCKED: u32 = 2;
    pub const TYPE_REMOVE: u32 = 3;
    pub const TYPE_REMOVE_ALL: u32 = 4;
}
