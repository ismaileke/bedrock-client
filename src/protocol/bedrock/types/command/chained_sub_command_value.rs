#[derive(serde::Serialize, Debug, Clone)]
pub struct ChainedSubCommandValue {
    name: String,
    arg_type: u16
}

impl ChainedSubCommandValue {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_arg_type(&self) -> u16 {
        self.arg_type
    }
}