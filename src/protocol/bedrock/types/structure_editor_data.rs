use crate::protocol::bedrock::types::structure_settings::StructureSettings;

#[derive(serde::Serialize, Debug)]
pub struct StructureEditorData {
    pub structure_name: String,
    pub filtered_structure_name: String,
    pub structure_data_field: String,
    pub include_players: bool,
    pub show_bounding_box: bool,
    pub structure_block_type: i32,
    pub structure_settings: StructureSettings,
    pub structure_redstone_save_mode: i32,
}

impl StructureEditorData {
    pub const TYPE_DATA: i32 = 0;
    pub const TYPE_SAVE: i32 = 1;
    pub const TYPE_LOAD: i32 = 2;
    pub const TYPE_CORNER: i32 = 3;
    pub const TYPE_INVALID: i32 = 4;
    pub const TYPE_EXPORT: i32 = 5;
}
