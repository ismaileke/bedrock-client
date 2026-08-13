use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct NPCRequest {
    pub actor_runtime_id: u64,
    pub request_type: u8,
    pub command_string: String,
    pub action_index: u8,
    pub scene_name: String,
}

impl Packet for NPCRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDNPCRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        stream.put_u8(self.request_type);
        PacketSerializer::put_string(stream, self.command_string.clone());
        stream.put_u8(self.action_index);
        PacketSerializer::put_string(stream, self.scene_name.clone());
    }

    fn decode(stream: &mut Reader) -> NPCRequest {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let request_type = stream.get_u8();
        let command_string = PacketSerializer::get_string(stream);
        let action_index = stream.get_u8();
        let scene_name = PacketSerializer::get_string(stream);

        NPCRequest {
            actor_runtime_id,
            request_type,
            command_string,
            action_index,
            scene_name,
        }
    }
}
