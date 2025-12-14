use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraTargetInstruction {
    pub target_center_offset: Option<Vec<f32>>,
    pub actor_unique_id: i64,
}

impl CameraTargetInstruction {
    pub fn new(
        target_center_offset: Option<Vec<f32>>,
        actor_unique_id: i64,
    ) -> CameraTargetInstruction {
        CameraTargetInstruction {
            target_center_offset,
            actor_unique_id,
        }
    }

    pub fn read(stream: &mut Stream) -> CameraTargetInstruction {
        let target_center_offset =
            PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let actor_unique_id = stream.get_i64_le();

        CameraTargetInstruction {
            target_center_offset,
            actor_unique_id,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::write_optional(stream, &self.target_center_offset, |s, v| {
            PacketSerializer::put_vector3(s, v.clone())
        });
        stream.put_i64_le(self.actor_unique_id);
    }
}
