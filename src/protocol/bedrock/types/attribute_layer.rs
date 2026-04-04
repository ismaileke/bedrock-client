use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::attribute_environment::AttributeEnvironment;
use crate::protocol::bedrock::types::attribute_layer_settings::AttributeLayerSettings;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct AttributeLayer {
    pub name: String,
    pub dimension: u32,
    pub settings: AttributeLayerSettings,
    pub attributes: Vec<AttributeEnvironment>
}

impl AttributeLayer {
    pub fn new(name: String, dimension: u32, settings: AttributeLayerSettings, attributes: Vec<AttributeEnvironment>) -> AttributeLayer {
        AttributeLayer { name, dimension, settings, attributes }
    }

    pub fn read(stream: &mut Stream) -> AttributeLayer {
        let name = PacketSerializer::get_string(stream);
        let dimension = stream.get_var_u32();
        let settings = AttributeLayerSettings::read(stream);
        let len = stream.get_var_u32() as usize;
        let mut attributes = Vec::with_capacity(len);
        for _ in 0..len {
            attributes.push(AttributeEnvironment::read(stream));
        }

        AttributeLayer { name, dimension, settings, attributes }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        stream.put_var_u32(self.dimension);
        self.settings.write(stream);
        stream.put_var_u32(self.attributes.len() as u32);
        for attribute in &self.attributes {
            attribute.write(stream);
        }
    }
}
