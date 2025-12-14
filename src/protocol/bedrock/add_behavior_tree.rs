use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct AddBehaviorTree {
    pub behavior_tree_json: String,
}

pub fn new(behavior_tree_json: String) -> AddBehaviorTree {
    AddBehaviorTree { behavior_tree_json }
}

impl Packet for AddBehaviorTree {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddBehaviorTree.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.behavior_tree_json.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> AddBehaviorTree {
        let behavior_tree_json = PacketSerializer::get_string(stream);

        AddBehaviorTree { behavior_tree_json }
    }

    fn debug(&self) {
        println!("Behavior Tree JSON: {}", self.behavior_tree_json);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
