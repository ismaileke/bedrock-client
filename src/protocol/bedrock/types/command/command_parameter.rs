use crate::protocol::bedrock::available_commands::AvailableCommands;
use crate::protocol::bedrock::types::command::command_enum::CommandEnum;

#[derive(serde::Serialize, Debug, Clone)]
pub struct CommandParameter {
    pub param_name: String,
    pub param_type: u32,
    pub is_optional: bool,
    pub flags: u8,
    pub command_enum: Option<CommandEnum>,
    pub postfix: Option<String>,
}

impl CommandParameter {
    pub const FLAG_FORCE_COLLAPSE_ENUM: u8 = 0x1;
    pub const FLAG_HAS_ENUM_CONSTRAINT: u8 = 0x2;

    fn baseline(
        param_name: String,
        param_type: u32,
        flags: u8,
        optional: bool,
    ) -> CommandParameter {
        CommandParameter {
            param_name,
            param_type,
            flags,
            is_optional: optional,
            command_enum: None,
            postfix: None,
        }
    }

    pub fn standard(
        param_name: String,
        param_type: u32,
        flags: u8,
        optional: bool,
    ) -> CommandParameter {
        CommandParameter::baseline(
            param_name,
            AvailableCommands::ARG_FLAG_VALID | param_type,
            flags,
            optional,
        )
    }

    pub fn post_fixed(
        param_name: String,
        postfix: String,
        flags: u8,
        optional: bool,
    ) -> CommandParameter {
        let mut result = CommandParameter::baseline(
            param_name,
            AvailableCommands::ARG_FLAG_POSTFIX,
            flags,
            optional,
        );
        result.postfix = Some(postfix);
        result
    }

    pub fn parameter_enum(
        param_name: String,
        command_enum: CommandEnum,
        flags: u8,
        optional: bool,
    ) -> CommandParameter {
        let mut result = CommandParameter::baseline(
            param_name,
            AvailableCommands::ARG_FLAG_ENUM | AvailableCommands::ARG_FLAG_VALID,
            flags,
            optional,
        );
        result.command_enum = Some(command_enum);
        result
    }
}
