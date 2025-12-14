use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct NPCDialogue {
    pub npc_actor_unique_id: i64,
    pub action_type: i32,
    pub dialogue: String,
    pub scene_name: String,
    pub npc_name: String,
    pub action_json: String,
}

pub fn new(
    npc_actor_unique_id: i64,
    action_type: i32,
    dialogue: String,
    scene_name: String,
    npc_name: String,
    action_json: String,
) -> NPCDialogue {
    NPCDialogue {
        npc_actor_unique_id,
        action_type,
        dialogue,
        scene_name,
        npc_name,
        action_json,
    }
}

impl Packet for NPCDialogue {
    fn id(&self) -> u16 {
        BedrockPacketType::IDNpcDialogue.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_i64_le(self.npc_actor_unique_id); // WHY??
        stream.put_var_i32(self.action_type);
        PacketSerializer::put_string(&mut stream, self.dialogue.clone());
        PacketSerializer::put_string(&mut stream, self.scene_name.clone());
        PacketSerializer::put_string(&mut stream, self.npc_name.clone());
        PacketSerializer::put_string(&mut stream, self.action_json.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> NPCDialogue {
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

    fn debug(&self) {
        println!("NPC Actor Unique ID: {}", self.npc_actor_unique_id);
        println!("Action Type: {}", self.action_type);
        println!("Dialogue: {}", self.dialogue);
        println!("Scene Name: {}", self.scene_name);
        println!("NPC Name: {}", self.npc_name);
        println!("Action JSON: {}", self.action_json);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl NPCDialogue {
    pub const ACTION_OPEN: i32 = 0;
    pub const ACTION_CLOSE: i32 = 1;
}
