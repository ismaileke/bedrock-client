use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CommandEnumConstraintRawData {
    pub affected_value_index: u32,
    pub enum_index: u32,
    pub constraints: Vec<u8>
}

impl CommandEnumConstraintRawData {
    pub fn new(affected_value_index: u32, enum_index: u32, constraints: Vec<u8>) -> CommandEnumConstraintRawData {
        CommandEnumConstraintRawData { affected_value_index, enum_index, constraints }
    }

    pub fn read(stream: &mut Stream) -> CommandEnumConstraintRawData {
        let affected_value_index = stream.get_var_u32();
        let enum_index = stream.get_var_u32();
        let constraints_size = stream.get_var_u32();
        let mut constraints = Vec::with_capacity(constraints_size as usize);
        for _ in 0..constraints_size {
            let constraint = stream.get_byte();
            constraints.push(constraint);
        }

        CommandEnumConstraintRawData { affected_value_index, enum_index, constraints }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.affected_value_index);
        stream.put_var_u32(self.enum_index);
        stream.put_var_u32(self.constraints.len() as u32);
        for constraint in &self.constraints {
            stream.put_byte(*constraint);
        }
    }
}
