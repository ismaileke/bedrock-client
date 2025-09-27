use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct AttributeModifier {
    pub id: String,
    pub name: String,
    pub amount: f32,
    pub operation: u32,
    pub operand: u32,
    pub serializable: bool
}

pub fn new(id: String, name: String, amount: f32, operation: u32, operand: u32, serializable: bool) -> AttributeModifier {
    AttributeModifier{ id, name, amount, operation, operand, serializable }
}

impl AttributeModifier {
    pub fn read(stream: &mut Stream) -> AttributeModifier {

        let id = PacketSerializer::get_string(stream);
        let name = PacketSerializer::get_string(stream);
        let amount = stream.get_l_float();
        let operation = stream.get_l_int();
        let operand = stream.get_l_int();
        let serializable = stream.get_bool();

        AttributeModifier{ id, name, amount, operation, operand, serializable }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.id.clone());
        PacketSerializer::put_string(stream, self.name.clone());
        stream.put_l_float(self.amount);
        stream.put_l_int(self.operation);
        stream.put_l_int(self.operand);
        stream.put_bool(self.serializable);
    }
}