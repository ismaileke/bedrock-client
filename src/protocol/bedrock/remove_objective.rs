use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct RemoveObjective {
    pub objective_name: String,
}

impl Packet for RemoveObjective {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRemoveObjective.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.objective_name);
    }

    fn decode(stream: &mut Reader) -> RemoveObjective {
        let objective_name = PacketSerializer::get_string(stream);

        RemoveObjective { objective_name }
    }
}
