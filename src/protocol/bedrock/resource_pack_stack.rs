use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::experiments::Experiments;
use crate::protocol::bedrock::types::resource_pack_stack_entry::ResourcePackStackEntry;

pub struct ResourcePackStack {
    pub resource_pack_stack: Vec<ResourcePackStackEntry>,
    pub behavior_pack_stack: Vec<ResourcePackStackEntry>,
    pub must_accept: bool,
    pub base_game_version: String,
    pub experiments: Experiments,
    pub use_vanilla_editor_packs: bool
}

impl ResourcePackStack {
    pub fn debug(&self) {
        for entry in &self.resource_pack_stack {
            println!("Resource Pack Stack - {:?}", entry);
        }
        for entry in &self.behavior_pack_stack {
            println!("Behavior Pack Stack - {:?}", entry);
        }
        println!("Must Accept: {}", self.must_accept);
        println!("Base Game Version: {}", self.base_game_version);
        println!("Experiments: {:?}", self.experiments);
        println!("Use Vanilla Editor Packs: {}", self.use_vanilla_editor_packs);
    }
}

pub fn decode(bytes: Vec<u8>) -> ResourcePackStack {
    let mut stream = Stream::new(bytes, 0);

    let must_accept = stream.get_bool();

    let mut behavior_pack_stack = vec![];
    let behavior_pack_count = stream.get_unsigned_var_int();
    for _ in 0..behavior_pack_count {
        behavior_pack_stack.push(ResourcePackStackEntry::read(&mut stream));
    }

    let mut resource_pack_stack = vec![];
    let resource_pack_count = stream.get_unsigned_var_int();
    for _ in 0..resource_pack_count {
        resource_pack_stack.push(ResourcePackStackEntry::read(&mut stream));
    }

    let length = stream.get_unsigned_var_int();
    let base_game_version = String::from_utf8(stream.get(length).unwrap()).unwrap();

    let experiments = Experiments::read(&mut stream);

    let use_vanilla_editor_packs = stream.get_bool();



    ResourcePackStack { resource_pack_stack, behavior_pack_stack, must_accept, base_game_version, experiments, use_vanilla_editor_packs }
}