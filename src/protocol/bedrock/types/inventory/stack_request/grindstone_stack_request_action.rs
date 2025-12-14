use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct GrindstoneStackRequestAction {
    recipe_id: u32,
    repair_cost: i32, //WHY
    repetitions: u8,
}

impl GrindstoneStackRequestAction {
    pub fn new(recipe_id: u32, repair_cost: i32, repetitions: u8) -> GrindstoneStackRequestAction {
        GrindstoneStackRequestAction {
            recipe_id,
            repair_cost,
            repetitions,
        }
    }

    pub fn read(stream: &mut Stream) -> GrindstoneStackRequestAction {
        let recipe_id = PacketSerializer::read_recipe_net_id(stream);
        let repair_cost = stream.get_var_i32();
        let repetitions = stream.get_byte();

        GrindstoneStackRequestAction {
            recipe_id,
            repair_cost,
            repetitions,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::write_recipe_net_id(stream, self.recipe_id);
        stream.put_var_i32(self.repair_cost);
        stream.put_byte(self.repetitions);
    }
}
