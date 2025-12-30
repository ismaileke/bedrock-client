use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct MobArmorEquipment {
    pub actor_runtime_id: u64,
    pub head: ItemStackWrapper,
    pub chest: ItemStackWrapper,
    pub legs: ItemStackWrapper,
    pub feet: ItemStackWrapper,
    pub body: ItemStackWrapper,
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

    fn decode(stream: &mut Stream) -> MobArmorEquipment {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let head = PacketSerializer::get_item_stack_wrapper(stream);
        let chest = PacketSerializer::get_item_stack_wrapper(stream);
        let legs = PacketSerializer::get_item_stack_wrapper(stream);
        let feet = PacketSerializer::get_item_stack_wrapper(stream);
        let body = PacketSerializer::get_item_stack_wrapper(stream);

        MobArmorEquipment {
            actor_runtime_id,
            head,
            chest,
            legs,
            feet,
            body,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
