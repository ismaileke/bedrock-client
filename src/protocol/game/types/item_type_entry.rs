pub struct ItemTypeEntry {
    string_id: String,
    numeric_id: i16,
    component_based: bool
}

impl ItemTypeEntry {
    pub fn new(string_id: String, numeric_id: i16, component_based: bool) -> ItemTypeEntry {
        ItemTypeEntry {
            string_id,
            numeric_id,
            component_based,
        }
    }

    pub fn get_string_id(&self) -> String {
        self.string_id.clone()
    }

    pub fn get_numeric_id(&self) -> i16 {
        self.numeric_id
    }

    pub fn is_component_based(&self) -> bool {
        self.component_based
    }
}