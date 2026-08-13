use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::command::command_soft_enum::CommandSoftEnum;
use crate::protocol::bedrock::types::command::raw::chained_sub_command_raw_data::ChainedSubCommandRawData;
use crate::protocol::bedrock::types::command::raw::command_enum_constraint_raw_data::CommandEnumConstraintRawData;
use crate::protocol::bedrock::types::command::raw::command_enum_raw_data::CommandEnumRawData;
use crate::protocol::bedrock::types::command::raw::command_raw_data::CommandRawData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AvailableCommands {
    pub enum_values: Vec<String>,
    pub chained_sub_command_values: Vec<String>,
    pub postfixes: Vec<String>,
    pub enums: Vec<CommandEnumRawData>,
    pub chained_sub_command_data: Vec<ChainedSubCommandRawData>,
    pub command_data: Vec<CommandRawData>,
    pub soft_enums: Vec<CommandSoftEnum>,
    pub enum_constraints: Vec<CommandEnumConstraintRawData>
}

impl AvailableCommands {
    /**
     * This flag is set on all types EXCEPT the POSTFIX type. Not completely sure what this is for, but it is required
     * for the argtype to work correctly. VALID seems as good a name as any.
     */
    pub const ARG_FLAG_VALID: u32 = 0x100000;
    /**
     * Enums are a little different: they are composed as follows:
     * ARG_FLAG_ENUM | ARG_FLAG_VALID | (enum index)
     */
    pub const ARG_FLAG_ENUM: u32 = 0x200000;

    /** This is used for /xp <level: int>L. It can only be applied to integer parameters. */
    pub const ARG_FLAG_POSTFIX: u32 = 0x1000000;

    pub const ARG_FLAG_SOFT_ENUM: u32 = 0x4000000;
}

impl Packet for AvailableCommands {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAvailableCommands.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.enum_values.len() as u32);
        for value in &self.enum_values {
            PacketSerializer::put_string(stream, value.clone());
        }
        stream.put_var_u32(self.chained_sub_command_values.len() as u32);
        for value in &self.chained_sub_command_values {
            PacketSerializer::put_string(stream, value.clone());
        }
        stream.put_var_u32(self.postfixes.len() as u32);
        for value in &self.postfixes {
            PacketSerializer::put_string(stream, value.clone());
        }
        stream.put_var_u32(self.enums.len() as u32);
        for value in &self.enums {
            value.write(stream);
        }
        stream.put_var_u32(self.chained_sub_command_data.len() as u32);
        for value in &self.chained_sub_command_data {
            value.write(stream);
        }
        stream.put_var_u32(self.command_data.len() as u32);
        for value in &self.command_data {
            value.write(stream);
        }
        stream.put_var_u32(self.soft_enums.len() as u32);
        for value in &self.soft_enums {
            value.write(stream);
        }
        stream.put_var_u32(self.enum_constraints.len() as u32);
        for value in &self.enum_constraints {
            value.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> AvailableCommands {
        let mut enum_values = Vec::new();
        let mut chained_sub_command_values = Vec::new();
        let mut postfixes = Vec::new();
        let mut enums = Vec::new();
        let mut chained_sub_command_data = Vec::new();
        let mut command_data = Vec::new();
        let mut soft_enums = Vec::new();
        let mut enum_constraints = Vec::new();
        let mut size = stream.get_var_u32();
        for _ in 0..size {
            enum_values.push(PacketSerializer::get_string(stream));
        }
        size = stream.get_var_u32();
        for _ in 0..size {
            chained_sub_command_values.push(PacketSerializer::get_string(stream));
        }
        size = stream.get_var_u32();
        for _ in 0..size {
            postfixes.push(PacketSerializer::get_string(stream));
        }
        size = stream.get_var_u32();
        for _ in 0..size {
            enums.push(CommandEnumRawData::read(stream));
        }
        size = stream.get_var_u32();
        for _ in 0..size {
            chained_sub_command_data.push(ChainedSubCommandRawData::read(stream));
        }
        size = stream.get_var_u32();
        for _ in 0..size {
            command_data.push(CommandRawData::read(stream));
        }
        size = stream.get_var_u32();
        for _ in 0..size {
            soft_enums.push(CommandSoftEnum::read(stream));
        }
        size = stream.get_var_u32();
        for _ in 0..size {
            enum_constraints.push(CommandEnumConstraintRawData::read(stream));
        }

        AvailableCommands {
            enum_values,
            chained_sub_command_values,
            postfixes,
            enums,
            chained_sub_command_data,
            command_data,
            soft_enums,
            enum_constraints,
        }
    }
}
