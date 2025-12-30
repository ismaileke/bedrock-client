use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::experiments::Experiments;
use crate::protocol::bedrock::types::resource_packs::resource_pack_stack_entry::ResourcePackStackEntry;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ResourcePackStack {
    pub resource_pack_stack: Vec<ResourcePackStackEntry>,
    pub must_accept: bool,
    pub base_game_version: String,
    pub experiments: Experiments,
    pub use_vanilla_editor_packs: bool,
}

impl Packet for ResourcePackStack {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackStack.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        todo!()
    }

    fn decode(stream: &mut Stream) -> ResourcePackStack {
        let must_accept = stream.get_bool();
        let mut resource_pack_stack = vec![];
        let resource_pack_count = stream.get_var_u32();
        for _ in 0..resource_pack_count {
            resource_pack_stack.push(ResourcePackStackEntry::read(stream));
        }
        let base_game_version = PacketSerializer::get_string(stream);
        let experiments = Experiments::read(stream);
        let use_vanilla_editor_packs = stream.get_bool();

        ResourcePackStack {
            resource_pack_stack,
            must_accept,
            base_game_version,
            experiments,
            use_vanilla_editor_packs,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
