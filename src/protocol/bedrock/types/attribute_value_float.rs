use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AttributeValueFloat {
    pub value: f32,
    pub operation: String
}

impl AttributeValueFloat {
    pub const OPERATION_OVERRIDE: &str = "override";
    pub const OPERATION_ALPHA_BLEND: &str = "alpha_blend";
    pub const OPERATION_ADD: &str = "add";
    pub const OPERATION_SUBTRACT: &str = "subtract";
    pub const OPERATION_MULTIPLY: &str = "multiply";
    pub const OPERATION_MINIMUM: &str = "minimum";
    pub const OPERATION_MAXIMUM: &str = "maximum";

    pub fn new(value: f32, operation: String) -> AttributeValueFloat {
        AttributeValueFloat { value, operation }
    }

    pub fn read(stream: &mut Reader) -> AttributeValueFloat {
        let value = stream.get_f32_le();
        let operation = PacketSerializer::get_string(stream);

        AttributeValueFloat { value, operation }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_f32_le(self.value);
        PacketSerializer::put_string(stream, &self.operation);
    }
}
