use crate::protocol::bedrock::types::command::command_enum::CommandEnum;

#[derive(Debug)]
pub struct CommandEnumConstraint {
    command_enum: CommandEnum,
    value_offset: u32,
    constraints: Vec<u8>
}

impl CommandEnumConstraint {
    pub const REQUIRES_CHEATS_ENABLED: u32 = 1 << 0;
    pub const REQUIRES_ELEVATED_PERMISSIONS: u32 = 1 << 1;
    pub const REQUIRES_HOST_PERMISSIONS: u32 = 1 << 2;
    pub const REQUIRES_ALLOW_ALIASES: u32 = 1 << 3;

    pub fn get_command_enum(&self) -> &CommandEnum {
        &self.command_enum
    }

    pub fn get_value_offset(&self) -> u32 {
        self.value_offset
    }

    pub fn get_constraints(&self) -> Vec<u8> {
        self.constraints.clone()
    }

    pub fn get_affected_value(&self) -> String {
        self.command_enum.get_enum_values()[self.get_value_offset() as usize].clone()
    }
}
