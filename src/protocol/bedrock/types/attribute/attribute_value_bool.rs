use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AttributeValueBool {
    pub value: bool,
    pub operation: String
}

impl AttributeValueBool {
    pub const OPERATION_OVERRIDE: &str = "override";
    pub const OPERATION_ALPHA_BLEND: &str = "alpha_blend";
    pub const OPERATION_AND: &str = "and";
    pub const OPERATION_NAND: &str = "nand";
    pub const OPERATION_OR: &str = "or";
    pub const OPERATION_NOR: &str = "nor";
    pub const OPERATION_XOR: &str = "xor";
    pub const OPERATION_XNOR: &str = "xnor";

    pub fn new(value: bool, operation: String) -> AttributeValueBool {
        AttributeValueBool { value, operation }
    }

    pub fn read(stream: &mut Reader) -> AttributeValueBool {
        let value = stream.get_bool();
        let operation = PacketSerializer::get_string(stream);

        AttributeValueBool { value, operation }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_bool(self.value);
        PacketSerializer::put_string(stream, &self.operation);
    }
}
