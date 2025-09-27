use std::any::Any;
use std::collections::HashMap;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;

pub struct AddItemActor {
    pub actor_unique_id: i64,
    pub actor_runtime_id: u64,
    pub item: ItemStackWrapper,
    pub position: Vec<f32>,
    pub motion: Vec<f32>,
    pub metadata: HashMap<u32, Box<dyn MetadataProperty>>,
    pub is_from_fishing: bool
}

pub fn new(actor_unique_id: i64, actor_runtime_id: u64, item: ItemStackWrapper, position: Vec<f32>, motion: Vec<f32>, metadata: HashMap<u32, Box<dyn MetadataProperty>>, is_from_fishing: bool) -> AddItemActor {
    AddItemActor { actor_unique_id, actor_runtime_id, item, position, motion, metadata, is_from_fishing }
}

impl Packet for AddItemActor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddItemActor.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.item.clone());
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        PacketSerializer::put_vector3_nullable(&mut stream, Option::from(self.motion.clone()));
        PacketSerializer::put_entity_metadata(&mut stream, &mut self.metadata);
        stream.put_bool(self.is_from_fishing);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> AddItemActor {
        let mut stream = Stream::new(bytes, 0);

        let actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let item = PacketSerializer::get_item_stack_wrapper(&mut stream);
        let position = PacketSerializer::get_vector3(&mut stream);
        let motion = PacketSerializer::get_vector3(&mut stream);
        let metadata = PacketSerializer::get_entity_metadata(&mut stream);
        let is_from_fishing = stream.get_bool();

        AddItemActor { actor_unique_id, actor_runtime_id, item, position, motion, metadata, is_from_fishing }
    }

    fn debug(&self) {
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Item: {:?}", self.item);
        println!("Position: {:?}", self.position);
        println!("Motion: {:?}", self.motion);
        println!("Metadata: {:?}", self.metadata);
        println!("Is from fishing: {}", self.is_from_fishing);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
