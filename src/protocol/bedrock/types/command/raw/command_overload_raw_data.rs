use crate::protocol::bedrock::types::command::raw::command_parameter_raw_data::CommandParameterRawData;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CommandOverloadRawData {
    pub chaining: bool,
    pub parameters: Vec<CommandParameterRawData>
}

impl CommandOverloadRawData {
    pub fn new(chaining: bool, parameters: Vec<CommandParameterRawData>) -> CommandOverloadRawData {
        CommandOverloadRawData { chaining, parameters }
    }

    pub fn read(stream: &mut Stream) -> CommandOverloadRawData {
        let chaining = stream.get_bool();
        let size = stream.get_var_u32();
        let mut parameters = Vec::with_capacity(size as usize);
        for _ in 0..size {
            parameters.push(CommandParameterRawData::read(stream));
        }

        CommandOverloadRawData { chaining, parameters }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_bool(self.chaining);
        stream.put_var_u32(self.parameters.len() as u32);
        for parameter in &self.parameters {
            parameter.write(stream);
        }
    }
}
