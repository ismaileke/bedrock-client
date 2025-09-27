use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct AddBehaviorTree {
    pub behavior_tree_json: String
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
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.behavior_tree_json.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> AddBehaviorTree {
        let mut stream = Stream::new(bytes, 0);

        let behavior_tree_json = PacketSerializer::get_string(&mut stream);

        AddBehaviorTree { behavior_tree_json }
    }

    fn debug(&self) {
        println!("Behavior Tree JSON: {}", self.behavior_tree_json);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
