use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;

#[derive(serde::Serialize, Debug)]
pub struct MobEquipment {
    pub actor_runtime_id: u64,
    pub item: ItemStackWrapper,
    pub inventory_slot: u8,
    pub hotbar_slot: u8,
    pub window_id: u8
}

pub fn new(actor_runtime_id: u64, item: ItemStackWrapper, inventory_slot: u8, hotbar_slot: u8, window_id: u8) -> MobEquipment {
    MobEquipment { actor_runtime_id, item, inventory_slot, hotbar_slot, window_id }
}

impl Packet for MobEquipment {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMobEquipment.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.item.clone());
        stream.put_byte(self.inventory_slot);
        stream.put_byte(self.hotbar_slot);
        stream.put_byte(self.window_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> MobEquipment {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let item = PacketSerializer::get_item_stack_wrapper(stream);
        let inventory_slot = stream.get_byte();
        let hotbar_slot = stream.get_byte();
        let window_id = stream.get_byte();

        MobEquipment { actor_runtime_id, item, inventory_slot, hotbar_slot, window_id }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Item: {:?}", self.item);
        println!("Inventory Slot: {}", self.inventory_slot);
        println!("Hotbar Slot: {}", self.hotbar_slot);
        println!("Window ID: {}", self.window_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
