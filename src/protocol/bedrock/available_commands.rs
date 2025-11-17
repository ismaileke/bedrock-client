use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct AvailableCommands {}

pub fn new() -> AvailableCommands {
    AvailableCommands { }
}

impl AvailableCommands {
    /**
     * This flag is set on all types EXCEPT the POSTFIX type. Not completely sure what this is for, but it is required
     * for the argtype to work correctly. VALID seems as good a name as any.
     */
    pub const ARG_FLAG_VALID: u32  = 0x100000;
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
        BedrockPacketType::IDAvailableCommands.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        // TODO

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(_bytes: Vec<u8>) -> AvailableCommands {
        //let mut stream = Stream::new(bytes, 0);

        // TODO

        AvailableCommands { }
    }

    fn debug(&self) {
        // TODO
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


/*use std::any::Any;
use std::collections::HashMap;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::command::command_data::CommandData;
use crate::protocol::bedrock::types::command::command_enum::CommandEnum;
use crate::protocol::bedrock::types::command::command_enum_constraint::CommandEnumConstraint;


pub struct AvailableCommands {
    pub command_data: Vec<CommandData>,
    pub hardcoded_enums: Vec<CommandEnum>,
    pub soft_enums: Vec<CommandEnum>,
    pub enum_constraints: Vec<CommandEnumConstraint>
}

pub fn new(
    command_data: Vec<CommandData>,
    hardcoded_enums: Vec<CommandEnum>,
    soft_enums: Vec<CommandEnum>,
    enum_constraints: Vec<CommandEnumConstraint>
) -> AvailableCommands {
    AvailableCommands { command_data, hardcoded_enums, soft_enums, enum_constraints }
}

impl AvailableCommands {
    /**
     * This flag is set on all types EXCEPT the POSTFIX type. Not completely sure what this is for, but it is required
     * for the argtype to work correctly. VALID seems as good a name as any.
     */
    pub const ARG_FLAG_VALID: u32  = 0x100000;
    /**
     * Enums are a little different: they are composed as follows:
     * ARG_FLAG_ENUM | ARG_FLAG_VALID | (enum index)
     */
    pub const ARG_FLAG_ENUM: u32 = 0x200000;

    /** This is used for /xp <level: int>L. It can only be applied to integer parameters. */
    pub const ARG_FLAG_POSTFIX: u32 = 0x1000000;

    pub const ARG_FLAG_SOFT_ENUM: u32 = 0x4000000;

    pub const HARDCODED_ENUM_NAMES: HashMap<&'static str, bool> = HashMap::from([("CommandName", true)]);

    /**
     * Command data is decoded without referencing to any soft enums, as they are decoded afterwards.
     * So we need to separately add soft enums to the command data
     */
    fn init_soft_enums_in_command_data(&self) {
        for datum in &self.command_data {
            for overload in &datum.get_overloads() {
                for parameter in overload.get_parameters().iter_mut() {
                    if parameter.param_type & Self::ARG_FLAG_SOFT_ENUM != 0 {
                        let index = parameter.param_type & 0xffff;
                        parameter.command_enum = self.soft_enums.get(index as usize).cloned();
                        if parameter.command_enum.is_none() {
                            panic!("Deserializing {} parameter {}: expected soft enum at {}, but got none", datum.get_name(), parameter.param_name, index);
                        }
                    }
                }
            }
        }
    }

    fn get_enum(&self, stream: &mut Stream, enum_value_list: HashMap<usize, String>) -> CommandEnum {
        let enum_name = PacketSerializer::get_string(stream);
        let mut enum_values = Vec::new();
        let list_size = enum_value_list.len();
        let len = stream.get_unsigned_var_int();
        for _ in 0..len {
            let index = Self::get_enum_value_index(list_size, stream);
            if enum_value_list.get(&index).is_none() {
                panic!("Invalid enum value index {}", index);
            }
            enum_values.push(enum_value_list.get(&index).unwrap().clone());
        }

        CommandEnum::new(enum_name, enum_values, false)
    }

    fn get_soft_enum(&self, stream: &mut Stream) -> CommandEnum {
        let enum_name = PacketSerializer::get_string(stream);
        let mut enum_values = Vec::new();
        let len = stream.get_unsigned_var_int();
        for _ in 0..len {
            enum_values.push(PacketSerializer::get_string(stream));
        }

        CommandEnum::new(enum_name, enum_values, true)
    }

    fn put_enum(&self, command_enum: &CommandEnum, enum_value_map: HashMap<String, usize>, stream: &mut Stream) {
        PacketSerializer::put_string(stream, command_enum.get_enum_name().clone());

        let values = command_enum.get_enum_values();
        stream.put_unsigned_var_int(values.len() as u32);
        let list_size = enum_value_map.len();
        for value in &values {
            if enum_value_map.get(value).is_none() {
                panic!("Enum value '{}' doesn't have a value index", value);
            }
            self.put_enum_value_index(*(enum_value_map.get(value).unwrap()), list_size, stream);
        }
    }

    fn put_soft_enum(&self, command_enum: &CommandEnum, stream: &mut Stream) {
        PacketSerializer::put_string(stream, command_enum.get_enum_name().clone());
        let values= command_enum.get_enum_values();
        stream.put_unsigned_var_int(values.len() as u32);
        for value in &values {
            PacketSerializer::put_string(stream, value.clone());
        }
    }

    fn get_enum_value_index(value_count: usize, stream: &mut Stream) -> usize {
        if value_count < 256 {
            stream.get_byte() as usize
        } else if value_count < 65536 {
            stream.get_l_short() as usize
        } else {
            stream.get_l_int() as usize
        }
    }

    fn put_enum_value_index(&self, index: usize, value_count: usize, stream: &mut Stream) {
        if value_count < 256 {
            stream.put_byte(index as u8);
        } else if value_count < 65536 {
            stream.put_l_short(index as u16);
        } else {
            stream.put_l_int(index as u32);
        }
    }

    fn get_enum_constraint(command_enums: Vec<CommandEnum>, enum_values: Vec<String>, stream: &mut Stream) -> CommandEnumConstraint {
        let value_index = stream.get_l_int() as usize;
        let enum_value = enum_values.get(value_index);
        if enum_value.is_none() {
            panic!("Enum constraint refers to unknown enum value index {}", value_index);
        }
        let enum_index = stream.get_l_int() as usize;
        let command_enum = command_enums.get(enum_index);
        if command_enum.is_none() {
            panic!("Enum constraint refers to unknown enum index {}", enum_index);
        }
        let value_offset = enum_values.get(value_index).unwrap();
        

    }
}

impl Packet for AvailableCommands {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAvailableCommands.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> AvailableCommands {
        let mut stream = Stream::new(bytes, 0);

        let actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);

        AvailableCommands { actor_unique_id }
    }

    fn debug(&self) {
        println!("Actor Unique ID: {}", self.actor_unique_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
*/