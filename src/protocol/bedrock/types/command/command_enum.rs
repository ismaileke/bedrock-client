#[derive(serde::Serialize, Debug, Clone)]
pub struct CommandEnum {
    pub enum_name: String,
    pub enum_values: Vec<String>,
    pub is_soft: bool,
}

impl CommandEnum {
    pub fn new(enum_name: String, enum_values: Vec<String>, is_soft: bool) -> CommandEnum {
        CommandEnum {
            enum_name,
            enum_values,
            is_soft,
        }
    }
    pub fn get_enum_name(&self) -> &String {
        &self.enum_name
    }

    pub fn get_enum_values(&self) -> &Vec<String> {
        &self.enum_values
    }

    pub fn get_is_soft(&self) -> bool {
        self.is_soft
    }
}
