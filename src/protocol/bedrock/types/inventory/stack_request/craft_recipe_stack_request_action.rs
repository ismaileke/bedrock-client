use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CraftRecipeStackRequestAction {
    recipe_id: u32,
    repetitions: u8,
}

impl CraftRecipeStackRequestAction {
    pub fn new(recipe_id: u32, repetitions: u8) -> CraftRecipeStackRequestAction {
        CraftRecipeStackRequestAction {
            recipe_id,
            repetitions,
        }
    }

    pub fn read(stream: &mut Stream) -> CraftRecipeStackRequestAction {
        let recipe_id = PacketSerializer::read_recipe_net_id(stream);
        let repetitions = stream.get_byte();

        CraftRecipeStackRequestAction {
            recipe_id,
            repetitions,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::write_recipe_net_id(stream, self.recipe_id);
        stream.put_byte(self.repetitions);
    }
}
