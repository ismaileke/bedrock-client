use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::camera::camera_spline_definition::CameraSplineDefinition;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CameraSpline {
    pub splines: Vec<CameraSplineDefinition>,
}

impl Packet for CameraSpline {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraSpline.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.splines.len() as u32);
        for spline in &self.splines {
            spline.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> CameraSpline {
        let count = stream.get_var_u32();
        let mut splines = Vec::new();
        for _ in 0..count {
            splines.push(CameraSplineDefinition::read(stream));
        }

        CameraSpline { splines }
    }
}
