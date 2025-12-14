use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct MotionPredictionHints {
    pub actor_runtime_id: u64,
    pub motion: Vec<f32>,
    pub on_ground: bool,
}

pub fn new(actor_runtime_id: u64, motion: Vec<f32>, on_ground: bool) -> MotionPredictionHints {
    MotionPredictionHints {
        actor_runtime_id,
        motion,
        on_ground,
    }
}

impl Packet for MotionPredictionHints {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMotionPredictionHints.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        PacketSerializer::put_vector3(&mut stream, self.motion.clone());
        stream.put_bool(self.on_ground);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> MotionPredictionHints {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let motion = PacketSerializer::get_vector3(stream);
        let on_ground = stream.get_bool();

        MotionPredictionHints {
            actor_runtime_id,
            motion,
            on_ground,
        }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Motion: {:?}", self.motion);
        println!("On Ground: {}", self.on_ground);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
