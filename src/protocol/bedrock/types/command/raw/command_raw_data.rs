use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::command::raw::command_overload_raw_data::CommandOverloadRawData;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CommandRawData {
    pub name: String,
    pub description: String,
    pub flags: u16,
    pub permission: String,
    pub alias_enum_index: i32,
    pub chained_sub_command_data_indexes: Vec<u32>,
    pub overloads: Vec<CommandOverloadRawData>
}

impl CommandRawData {
    pub fn new(
        name: String,
        description: String,
        flags: u16,
        permission: String,
        alias_enum_index: i32,
        chained_sub_command_data_indexes: Vec<u32>,
        overloads: Vec<CommandOverloadRawData>
    ) -> CommandRawData {
        CommandRawData { name, description, flags, permission, alias_enum_index, chained_sub_command_data_indexes, overloads }
    }

    pub fn read(stream: &mut Stream) -> CommandRawData {
        let name = PacketSerializer::get_string(stream);
        let description = PacketSerializer::get_string(stream);
        let flags = stream.get_u16_le();
        let permission = PacketSerializer::get_string(stream);
        let alias_enum_index = stream.get_i32_le();
        let mut size = stream.get_var_u32();
        let mut chained_sub_command_data_indexes = Vec::with_capacity(size as usize);
        for _ in 0..size {
            chained_sub_command_data_indexes.push(stream.get_u32_le());
        }
        size = stream.get_var_u32();
        let mut overloads = Vec::with_capacity(size as usize);
        for _ in 0..size {
            overloads.push(CommandOverloadRawData::read(stream));
        }

        CommandRawData { name, description, flags, permission, alias_enum_index, chained_sub_command_data_indexes, overloads }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        PacketSerializer::put_string(stream, self.description.clone());
        stream.put_u16_le(self.flags);
        PacketSerializer::put_string(stream, self.permission.clone());
        stream.put_i32_le(self.alias_enum_index);
        stream.put_var_u32(self.chained_sub_command_data_indexes.len() as u32);
        for index in &self.chained_sub_command_data_indexes {
            stream.put_u32_le(*index);
        }
        stream.put_var_u32(self.overloads.len() as u32);
        for overload in &self.overloads {
            overload.write(stream);
        }
    }
}
