use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::attribute_environment::AttributeEnvironment;
use crate::protocol::bedrock::types::attribute_layer_settings::AttributeLayerSettings;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AttributeLayer {
    pub name: String,
    pub noise_name: Option<String>,
    pub dimension: u32,
    pub settings: AttributeLayerSettings,
    pub attributes: Vec<AttributeEnvironment>
}

impl AttributeLayer {
    pub fn new(name: String, noise_name: Option<String>, dimension: u32, settings: AttributeLayerSettings, attributes: Vec<AttributeEnvironment>) -> AttributeLayer {
        AttributeLayer { name, noise_name, dimension, settings, attributes }
    }

    pub fn read(stream: &mut Reader) -> AttributeLayer {
        let name = PacketSerializer::get_string(stream);
        let noise_name = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let dimension = stream.get_var_u32();
        let settings = AttributeLayerSettings::read(stream);
        let len = stream.get_var_u32() as usize;
        let mut attributes = Vec::with_capacity(len);
        for _ in 0..len {
            attributes.push(AttributeEnvironment::read(stream));
        }

        AttributeLayer { name, noise_name, dimension, settings, attributes }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.name.clone());
        PacketSerializer::write_optional(stream, &self.noise_name, |s, v| PacketSerializer::put_string(s, v.clone()));
        stream.put_var_u32(self.dimension);
        self.settings.write(stream);
        stream.put_var_u32(self.attributes.len() as u32);
        for attribute in &self.attributes {
            attribute.write(stream);
        }
    }
}
