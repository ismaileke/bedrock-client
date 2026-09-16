use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct FurnaceOptions {
    pub left_furnace_tab: i32,
    pub filtering: bool,
    pub furnace_layout: i32
}

impl FurnaceOptions {
    pub const FURNACE_TAB_NONE: i32 = 0;
    pub const FURNACE_TAB_RECIPE_FOOD: i32 = 1;
    pub const FURNACE_TAB_RECIPE_ITEMS: i32 = 2;
    pub const FURNACE_TAB_RECIPE_BLOCKS: i32 = 3;
    pub const FURNACE_TAB_RECIPE_SEARCH: i32 = 4;
    pub const FURNACE_TAB_INVENTORY: i32 = 5;

    pub const FURNACE_LAYOUT_NONE: i32 = 0;
    pub const FURNACE_LAYOUT_INVENTORY_ONLY: i32 = 1;
    pub const FURNACE_LAYOUT_DEFAULT: i32 = 2;

    pub fn new(left_furnace_tab: i32, filtering: bool, furnace_layout: i32) -> FurnaceOptions {
        FurnaceOptions { left_furnace_tab, filtering, furnace_layout }
    }

    pub fn read(stream: &mut Reader) -> FurnaceOptions {
        let left_furnace_tab = stream.get_var_i32();
        let filtering = stream.get_bool();
        let furnace_layout = stream.get_var_i32();
        FurnaceOptions { left_furnace_tab, filtering, furnace_layout }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_i32(self.left_furnace_tab);
        stream.put_bool(self.filtering);
        stream.put_var_i32(self.furnace_layout);
    }
}
