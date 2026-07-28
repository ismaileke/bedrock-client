use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CommandParameterRawData {
    pub name: String,
    pub type_info: u32,
    pub optional: bool,
    pub flags: u8
}

impl CommandParameterRawData {
    pub fn new(name: String, type_info: u32, optional: bool, flags: u8) -> CommandParameterRawData {
        CommandParameterRawData { name, type_info, optional, flags }
    }

    pub fn read(stream: &mut Stream) -> CommandParameterRawData {
        let name = PacketSerializer::get_string(stream);
        let type_info = stream.get_u32_le();
        let optional = stream.get_bool();
        let flags = stream.get_byte();

        CommandParameterRawData { name, type_info, optional, flags }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        stream.put_u32_le(self.type_info);
        stream.put_bool(self.optional);
        stream.put_byte(self.flags);
    }
}
