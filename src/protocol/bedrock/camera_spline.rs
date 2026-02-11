use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::camera::camera_spline_definition::CameraSplineDefinition;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct CameraSpline {
    pub splines: Vec<CameraSplineDefinition>,
}

impl Packet for CameraSpline {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraSpline.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.splines.len() as u32);
        for spline in &self.splines {
            spline.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CameraSpline {
        let count = stream.get_var_u32();
        let mut splines = Vec::new();
        for _ in 0..count {
            splines.push(CameraSplineDefinition::read(stream));
        }

        CameraSpline { splines }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
