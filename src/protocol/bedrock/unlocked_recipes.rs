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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_int(self.unlock_type);
        stream.put_unsigned_var_int(self.recipes.len() as u32);
        for recipe in self.recipes.iter() {
            PacketSerializer::put_string(&mut stream, recipe.clone());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> UnlockedRecipes {
        let mut stream = Stream::new(bytes, 0);

        let unlock_type = stream.get_l_int();
        let recipes_len = stream.get_unsigned_var_int() as usize;
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
