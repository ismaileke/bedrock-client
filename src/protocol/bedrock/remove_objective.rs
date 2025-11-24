use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct RemoveObjective {
    pub objective_name: String
}

pub fn new(objective_name: String) -> RemoveObjective {
    RemoveObjective { objective_name }
}

impl Packet for RemoveObjective {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRemoveObjective.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.objective_name.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> RemoveObjective {
        let objective_name = PacketSerializer::get_string(stream);

        RemoveObjective { objective_name }
    }

    fn debug(&self) {
        println!("Objective Name: {}", self.objective_name);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
