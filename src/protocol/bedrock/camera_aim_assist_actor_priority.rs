use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::camera::camera_aim_assist_actor_priority_data::CameraAimAssistActorPriorityData;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct CameraAimAssistActorPriority {
    pub priority_data: Vec<CameraAimAssistActorPriorityData>,
}

impl Packet for CameraAimAssistActorPriority {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraAimAssistActorPriority.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.priority_data.len() as u32);
        for priority_datum in &self.priority_data {
            priority_datum.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CameraAimAssistActorPriority {
        let count = stream.get_var_u32();
        let mut priority_data = Vec::new();
        for _ in 0..count {
            priority_data.push(CameraAimAssistActorPriorityData::read(stream));
        }

        CameraAimAssistActorPriority { priority_data }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
