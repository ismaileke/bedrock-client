use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct MineBlockStackRequestAction {
    hotbar_slot: i32,
    predicted_durability: i32,
    stack_id: i32,
}

impl MineBlockStackRequestAction {
    pub fn new(
        hotbar_slot: i32,
        predicted_durability: i32,
        stack_id: i32,
    ) -> MineBlockStackRequestAction {
        MineBlockStackRequestAction {
            hotbar_slot,
            predicted_durability,
            stack_id,
        }
    }

    pub fn read(stream: &mut Stream) -> MineBlockStackRequestAction {
        let hotbar_slot = stream.get_var_i32();
        let predicted_durability = stream.get_var_i32();
        let stack_id = PacketSerializer::read_item_stack_net_id_variant(stream);

        MineBlockStackRequestAction {
            hotbar_slot,
            predicted_durability,
            stack_id,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_var_i32(self.hotbar_slot);
        stream.put_var_i32(self.predicted_durability);
        PacketSerializer::write_item_stack_net_id_variant(stream, self.stack_id);
    }
}
