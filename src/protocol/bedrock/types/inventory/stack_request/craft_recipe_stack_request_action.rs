use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CraftRecipeStackRequestAction {
    pub recipe_id: i32,
    pub repetitions: u8,
}

impl CraftRecipeStackRequestAction {
    pub fn new(recipe_id: i32, repetitions: u8) -> CraftRecipeStackRequestAction {
        CraftRecipeStackRequestAction { recipe_id, repetitions }
    }

    pub fn read(stream: &mut Reader) -> CraftRecipeStackRequestAction {
        let recipe_id = PacketSerializer::read_recipe_net_id(stream);
        let repetitions = stream.get_u8();

        CraftRecipeStackRequestAction { recipe_id, repetitions }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::write_recipe_net_id(stream, self.recipe_id);
        stream.put_u8(self.repetitions);
    }
}
