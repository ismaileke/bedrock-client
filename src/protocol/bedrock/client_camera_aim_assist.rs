use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct ClientCameraAimAssist {
    pub preset_id: String,
    pub action_type: u8, //see types/camera/camera_aim_assist_action_type.rs
    pub allow_aim_assist: bool
}

pub fn new(preset_id: String, action_type: u8, allow_aim_assist: bool) -> ClientCameraAimAssist {
    ClientCameraAimAssist { preset_id, action_type, allow_aim_assist }
}

impl Packet for ClientCameraAimAssist {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientCameraAimAssist.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.preset_id.clone());
        stream.put_byte(self.action_type);
        stream.put_bool(self.allow_aim_assist);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ClientCameraAimAssist {
        let preset_id = PacketSerializer::get_string(stream);
        let action_type = stream.get_byte();
        let allow_aim_assist = stream.get_bool();

        ClientCameraAimAssist { preset_id, action_type, allow_aim_assist }
    }

    fn debug(&self) {
        println!("Preset ID: {}", self.preset_id);
        println!("Action Type: {}", self.action_type);
        println!("Allow Aim Assist: {}", self.allow_aim_assist);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
