use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct AnimateEntity {
    pub animation: String,
    pub next_state: String,
    pub stop_expression: String,
    pub stop_expression_version: u32,
    pub controller: String,
    pub blend_out_time: f32,
    pub actor_runtime_ids: Vec<u64>
}

pub fn new(animation: String, next_state: String, stop_expression: String, stop_expression_version: u32, controller: String, blend_out_time: f32, actor_runtime_ids: Vec<u64>) -> AnimateEntity {
    AnimateEntity { animation, next_state, stop_expression, stop_expression_version, controller, blend_out_time, actor_runtime_ids }
}

impl Packet for AnimateEntity {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAnimateEntity.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.animation.clone());
        PacketSerializer::put_string(&mut stream, self.next_state.clone());
        PacketSerializer::put_string(&mut stream, self.stop_expression.clone());
        stream.put_l_int(self.stop_expression_version);
        PacketSerializer::put_string(&mut stream, self.controller.clone());
        stream.put_l_float(self.blend_out_time);
       stream.put_unsigned_var_int(self.actor_runtime_ids.len() as u32);
        for actor_runtime_id in self.actor_runtime_ids.iter() {
            PacketSerializer::put_actor_runtime_id(&mut stream, *actor_runtime_id);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> AnimateEntity {
        let mut stream = Stream::new(bytes, 0);

        let animation = PacketSerializer::get_string(&mut stream);
        let next_state = PacketSerializer::get_string(&mut stream);
        let stop_expression = PacketSerializer::get_string(&mut stream);
        let stop_expression_version = stream.get_l_int();
        let controller = PacketSerializer::get_string(&mut stream);
        let blend_out_time = stream.get_l_float();
        let actor_runtime_ids_len = stream.get_unsigned_var_int() as usize;
        let mut actor_runtime_ids = Vec::new();
        for _ in 0..actor_runtime_ids_len {
            actor_runtime_ids.push(PacketSerializer::get_actor_runtime_id(&mut stream));
        }

        AnimateEntity { animation, next_state, stop_expression, stop_expression_version, controller, blend_out_time, actor_runtime_ids }
    }

    fn debug(&self) {
        println!("Animation: {}", self.animation);
        println!("Next State: {}", self.next_state);
        println!("Stop Expression: {}", self.stop_expression);
        println!("Stop Expression Version: {}", self.stop_expression_version);
        println!("Controller: {}", self.controller);
        println!("Blend Out Time: {}", self.blend_out_time);
        println!("Actor Runtime IDs: {:?}", self.actor_runtime_ids);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
