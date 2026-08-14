use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::attribute_layer_settings::AttributeLayerSettings;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AttributeUpdateLayerSettings {
    pub name: String,
    pub dimension: u32,
    pub settings: AttributeLayerSettings
}

impl AttributeUpdateLayerSettings {

    pub fn new(name: String, dimension: u32, settings: AttributeLayerSettings) -> AttributeUpdateLayerSettings {
        AttributeUpdateLayerSettings { name, dimension, settings }
    }

    pub fn read(stream: &mut Reader) -> AttributeUpdateLayerSettings {
        let name = PacketSerializer::get_string(stream);
        let dimension = stream.get_var_u32();
        let settings = AttributeLayerSettings::read(stream);

        AttributeUpdateLayerSettings { name, dimension, settings }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.name);
        stream.put_var_u32(self.dimension);
        self.settings.write(stream);
    }
}
