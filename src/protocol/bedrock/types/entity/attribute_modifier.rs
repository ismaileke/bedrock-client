use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct AttributeModifier {
    pub id: String,
    pub name: String,
    pub amount: f32,
    pub operation: i32,
    pub operand: i32,
    pub serializable: bool
}

pub fn new(id: String, name: String, amount: f32, operation: i32, operand: i32, serializable: bool) -> AttributeModifier {
    AttributeModifier{ id, name, amount, operation, operand, serializable }
}

impl AttributeModifier {
    pub fn read(stream: &mut Stream) -> AttributeModifier {

        let id = PacketSerializer::get_string(stream);
        let name = PacketSerializer::get_string(stream);
        let amount = stream.get_f32_le();
        let operation = stream.get_i32_le();
        let operand = stream.get_i32_le();
        let serializable = stream.get_bool();

        AttributeModifier{ id, name, amount, operation, operand, serializable }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.id.clone());
        PacketSerializer::put_string(stream, self.name.clone());
        stream.put_f32_le(self.amount);
        stream.put_i32_le(self.operation);
        stream.put_i32_le(self.operand);
        stream.put_bool(self.serializable);
    }
}