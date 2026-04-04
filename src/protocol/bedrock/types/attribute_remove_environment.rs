use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct AttributeRemoveEnvironment {
    pub name: String,
    pub dimension: u32,
    pub attributes: Vec<String>
}

impl AttributeRemoveEnvironment {

    pub fn new(name: String, dimension: u32, attributes: Vec<String>) -> AttributeRemoveEnvironment {
        AttributeRemoveEnvironment { name, dimension, attributes }
    }

    pub fn read(stream: &mut Stream) -> AttributeRemoveEnvironment {
        let name = PacketSerializer::get_string(stream);
        let dimension = stream.get_var_u32();
        let len = stream.get_var_u32();
        let mut attributes = Vec::with_capacity(len as usize);
        for _ in 0..len {
            attributes.push(PacketSerializer::get_string(stream));
        }

        AttributeRemoveEnvironment { name, dimension, attributes }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        stream.put_var_u32(self.dimension);
        stream.put_var_u32(self.attributes.len() as u32);
        for attribute in &self.attributes {
            PacketSerializer::put_string(stream, attribute.clone());
        }
    }
}
