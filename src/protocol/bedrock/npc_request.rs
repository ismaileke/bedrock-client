use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct NPCRequest {
    pub actor_runtime_id: u64,
    pub request_type: u8,
    pub command_string: String,
    pub action_index: u8,
    pub scene_name: String,
}

pub fn new(
    actor_runtime_id: u64,
    request_type: u8,
    command_string: String,
    action_index: u8,
    scene_name: String,
) -> NPCRequest {
    NPCRequest {
        actor_runtime_id,
        request_type,
        command_string,
        action_index,
        scene_name,
    }
}

impl Packet for NPCRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDNPCRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_byte(self.request_type);
        PacketSerializer::put_string(&mut stream, self.command_string.clone());
        stream.put_byte(self.action_index);
        PacketSerializer::put_string(&mut stream, self.scene_name.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> NPCRequest {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let request_type = stream.get_byte();
        let command_string = PacketSerializer::get_string(stream);
        let action_index = stream.get_byte();
        let scene_name = PacketSerializer::get_string(stream);

        NPCRequest {
            actor_runtime_id,
            request_type,
            command_string,
            action_index,
            scene_name,
        }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Request Type: {}", self.request_type);
        println!("Command String: {}", self.command_string);
        println!("Action Index: {}", self.action_index);
        println!("Scene Name: {}", self.scene_name);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
