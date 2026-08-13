use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AddBehaviorTree {
    pub behavior_tree_json: String,
}

impl Packet for AddBehaviorTree {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddBehaviorTree.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.behavior_tree_json.clone());
    }

    fn decode(stream: &mut Reader) -> AddBehaviorTree {
        let behavior_tree_json = PacketSerializer::get_string(stream);

        AddBehaviorTree { behavior_tree_json }
    }
}
