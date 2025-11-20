use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct SetActorMotion {
    pub actor_runtime_id: u64,
    pub motion: Vec<f32>,
    pub tick: u64
}

pub fn new(actor_runtime_id: u64, motion: Vec<f32>, tick: u64) -> SetActorMotion {
    SetActorMotion { actor_runtime_id, motion, tick }
}

impl Packet for SetActorMotion {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetActorMotion.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        PacketSerializer::put_vector3(&mut stream, self.motion.clone());
        stream.put_var_u64(self.tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SetActorMotion {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let motion = PacketSerializer::get_vector3(stream);
        let tick = stream.get_var_u64();

        SetActorMotion { actor_runtime_id, motion, tick }
    }
    
    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Motion: {:?}", self.motion);
        println!("Tick: {}", self.tick);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
