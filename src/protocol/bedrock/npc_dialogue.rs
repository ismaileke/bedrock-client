use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct NPCDialogue {
    pub npc_actor_unique_id: i64,
    pub action_type: i32,
    pub dialogue: String,
    pub scene_name: String,
    pub npc_name: String,
    pub action_json: String,
}

impl Packet for NPCDialogue {
    fn id(&self) -> u16 {
        BedrockPacketType::IDNpcDialogue.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_i64_le(self.npc_actor_unique_id); // WHY??
        stream.put_var_i32(self.action_type);
        PacketSerializer::put_string(stream, &self.dialogue);
        PacketSerializer::put_string(stream, &self.scene_name);
        PacketSerializer::put_string(stream, &self.npc_name);
        PacketSerializer::put_string(stream, &self.action_json);
    }

    fn decode(stream: &mut Reader) -> NPCDialogue {
        let npc_actor_unique_id = stream.get_i64_le();
        let action_type = stream.get_var_i32();
        let dialogue = PacketSerializer::get_string(stream);
        let scene_name = PacketSerializer::get_string(stream);
        let npc_name = PacketSerializer::get_string(stream);
        let action_json = PacketSerializer::get_string(stream);

        NPCDialogue {
            npc_actor_unique_id,
            action_type,
            dialogue,
            scene_name,
            npc_name,
            action_json,
        }
    }
}

impl NPCDialogue {
    pub const ACTION_OPEN: i32 = 0;
    pub const ACTION_CLOSE: i32 = 1;
}
