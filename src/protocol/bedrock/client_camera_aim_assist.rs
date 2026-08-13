use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientCameraAimAssist {
    pub preset_id: String,
    pub action_type: u8, //see types/camera/camera_aim_assist_action_type.rs
    pub allow_aim_assist: bool
}

impl Packet for ClientCameraAimAssist {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientCameraAimAssist.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.preset_id.clone());
        stream.put_u8(self.action_type);
        stream.put_bool(self.allow_aim_assist);
    }

    fn decode(stream: &mut Reader) -> ClientCameraAimAssist {
        let preset_id = PacketSerializer::get_string(stream);
        let action_type = stream.get_u8();
        let allow_aim_assist = stream.get_bool();

        ClientCameraAimAssist { preset_id, action_type, allow_aim_assist }
    }
}
