use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct AttributeLayerSettingsWeightFloat {
    pub value: f32,
}

impl AttributeLayerSettingsWeightFloat {

    pub fn new(value: f32) -> AttributeLayerSettingsWeightFloat {
        AttributeLayerSettingsWeightFloat {
            value,
        }
    }

    pub fn read(stream: &mut Stream) -> AttributeLayerSettingsWeightFloat {
        AttributeLayerSettingsWeightFloat {
            value: stream.get_f32_le(),
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.value);
    }
}
