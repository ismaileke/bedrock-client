use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MineBlockStackRequestAction {
    pub hotbar_slot: i32,
    pub predicted_durability: i32,
    pub stack_id: i32,
}

impl MineBlockStackRequestAction {
    pub fn new(hotbar_slot: i32, predicted_durability: i32, stack_id: i32) -> MineBlockStackRequestAction {
        MineBlockStackRequestAction { hotbar_slot, predicted_durability, stack_id }
    }

    pub fn read(stream: &mut Reader) -> MineBlockStackRequestAction {
        let hotbar_slot = stream.get_var_i32();
        let predicted_durability = stream.get_var_i32();
        let stack_id = stream.get_i32_le();

        MineBlockStackRequestAction { hotbar_slot, predicted_durability, stack_id }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_i32(self.hotbar_slot);
        stream.put_var_i32(self.predicted_durability);
        stream.put_i32_le(self.stack_id);
    }
}
