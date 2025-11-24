use crate::protocol::bedrock::types::command::chained_sub_command_value::ChainedSubCommandValue;

#[derive(serde::Serialize, Debug, Clone)]
pub struct ChainedSubCommandData {
    name: String,
    values: Vec<ChainedSubCommandValue>
}

impl ChainedSubCommandData{
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_values(&self) -> Vec<ChainedSubCommandValue> {
        self.values.clone()
    }
}
