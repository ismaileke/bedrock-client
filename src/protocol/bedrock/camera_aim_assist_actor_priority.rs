use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::camera::camera_aim_assist_actor_priority_data::CameraAimAssistActorPriorityData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CameraAimAssistActorPriority {
    pub priority_data: Vec<CameraAimAssistActorPriorityData>,
}

impl Packet for CameraAimAssistActorPriority {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraAimAssistActorPriority.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.priority_data.len() as u32);
        for priority_datum in &self.priority_data {
            priority_datum.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> CameraAimAssistActorPriority {
        let count = stream.get_var_u32();
        let mut priority_data = Vec::new();
        for _ in 0..count {
            priority_data.push(CameraAimAssistActorPriorityData::read(stream));
        }

        CameraAimAssistActorPriority { priority_data }
    }
}
