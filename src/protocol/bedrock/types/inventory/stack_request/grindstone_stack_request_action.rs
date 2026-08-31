use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct GrindstoneStackRequestAction {
    pub recipe_id: i32,
    pub repair_cost: i32, //WHY
    pub repetitions: u8,
}

impl GrindstoneStackRequestAction {
    pub fn new(recipe_id: i32, repair_cost: i32, repetitions: u8) -> GrindstoneStackRequestAction {
        GrindstoneStackRequestAction { recipe_id, repair_cost, repetitions }
    }

    pub fn read(stream: &mut Reader) -> GrindstoneStackRequestAction {
        let recipe_id = PacketSerializer::read_recipe_net_id(stream);
        let repair_cost = stream.get_var_i32();
        let repetitions = stream.get_u8();

        GrindstoneStackRequestAction { recipe_id, repair_cost, repetitions }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::write_recipe_net_id(stream, self.recipe_id);
        stream.put_var_i32(self.repair_cost);
        stream.put_u8(self.repetitions);
    }
}
