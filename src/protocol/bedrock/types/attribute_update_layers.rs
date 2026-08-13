use crate::protocol::bedrock::types::attribute_layer::AttributeLayer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AttributeUpdateLayers {
    pub layers: Vec<AttributeLayer>
}

impl AttributeUpdateLayers {

    pub fn new(layers: Vec<AttributeLayer>) -> AttributeUpdateLayers {
        AttributeUpdateLayers { layers }
    }

    pub fn read(stream: &mut Reader) -> AttributeUpdateLayers {
        let len = stream.get_var_u32();
        let mut layers = Vec::with_capacity(len as usize);
        for _ in 0..len {
            layers.push(AttributeLayer::read(stream));
        }

        AttributeUpdateLayers { layers }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.layers.len() as u32);
        for layer in &self.layers {
            layer.write(stream);
        }
    }
}
