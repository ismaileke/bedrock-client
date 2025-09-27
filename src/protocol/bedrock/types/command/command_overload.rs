use crate::protocol::bedrock::types::command::command_parameter::CommandParameter;

#[derive(Debug, Clone)]
pub struct CommandOverload {
    chaining: bool,
    parameters: Vec<CommandParameter>
}

impl CommandOverload {
    pub fn is_chaining(&self) -> bool {
        self.chaining
    }

    pub fn get_parameters(&self) -> Vec<CommandParameter> {
        self.parameters.clone()
    }
}