use crate::protocol::bedrock::types::command::chained_sub_command_data::ChainedSubCommandData;
use crate::protocol::bedrock::types::command::command_enum::CommandEnum;
use crate::protocol::bedrock::types::command::command_overload::CommandOverload;

#[derive(serde::Serialize, Debug, Clone)]
pub struct CommandData {
    name: String,
    description: String,
    flags: u16,
    permission: u8,
    aliases: Option<CommandEnum>,
    overloads: Vec<CommandOverload>,
    chained_sub_command_data: Vec<ChainedSubCommandData>
}

impl CommandData {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_flags(&self) -> u16 {
        self.flags
    }

    pub fn get_permission(&self) -> u8 {
        self.permission
    }

    pub fn get_aliases(&self) -> Option<CommandEnum> {
        self.aliases.clone()
    }

    pub fn get_overloads(&self) -> Vec<CommandOverload> {
        self.overloads.clone()
    }

    pub fn get_chained_sub_command_data(&self) -> Vec<ChainedSubCommandData> {
        self.chained_sub_command_data.clone()
    }
}