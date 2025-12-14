use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct ParameterKeyframeValue {
    pub time: f32,
    pub value: Vec<f32>,
}

impl ParameterKeyframeValue {
    pub fn read(stream: &mut Stream) -> ParameterKeyframeValue {
        let time = stream.get_f32_le();
        let value = PacketSerializer::get_vector3(stream);

        ParameterKeyframeValue { time, value }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.time);
        PacketSerializer::put_vector3(stream, self.value.clone());
    }
}
