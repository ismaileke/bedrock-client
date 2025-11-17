use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;

pub struct MobArmorEquipment {
    pub actor_runtime_id: u64,
    pub head: ItemStackWrapper,
    pub chest: ItemStackWrapper,
    pub legs: ItemStackWrapper,
    pub feet: ItemStackWrapper,
    pub body: ItemStackWrapper
}

pub fn new(
    actor_runtime_id: u64,
    head: ItemStackWrapper,
    chest: ItemStackWrapper,
    legs: ItemStackWrapper,
    feet: ItemStackWrapper,
    body: ItemStackWrapper
) -> MobArmorEquipment {
    MobArmorEquipment { actor_runtime_id, head, chest, legs, feet, body }
}

impl Packet for MobArmorEquipment {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMobArmorEquipment.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.head.clone());
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.chest.clone());
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.legs.clone());
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.feet.clone());
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.body.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> MobArmorEquipment {
        let mut stream = Stream::new(bytes, 0);

        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let head = PacketSerializer::get_item_stack_wrapper(&mut stream);
        let chest = PacketSerializer::get_item_stack_wrapper(&mut stream);
        let legs = PacketSerializer::get_item_stack_wrapper(&mut stream);
        let feet = PacketSerializer::get_item_stack_wrapper(&mut stream);
        let body = PacketSerializer::get_item_stack_wrapper(&mut stream);

        MobArmorEquipment { actor_runtime_id, head, chest, legs, feet, body }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Head: {:?}", self.head);
        println!("Chest: {:?}", self.chest);
        println!("Legs: {:?}", self.legs);
        println!("Feet: {:?}", self.feet);
        println!("Body: {:?}", self.body);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
