use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CompletedUsingItem {
    pub item_id: i16,
    pub action: i32
}

pub fn new(item_id: i16, action: i32) -> CompletedUsingItem {
    CompletedUsingItem { item_id, action }
}

impl Packet for CompletedUsingItem {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCompletedUsingItem.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_i16_le(self.item_id);
        stream.put_i32_le(self.action);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CompletedUsingItem {
        let item_id = stream.get_i16_le();
        let action = stream.get_i32_le();

        CompletedUsingItem { item_id, action }
    }

    fn debug(&self) {
        println!("Item ID: {}", self.item_id);
        println!("Action: {}", self.action);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl CompletedUsingItem {
    pub const ACTION_UNKNOWN: i32 = -1;
    pub const ACTION_EQUIP_ARMOR: i32 = 0;
    pub const ACTION_EAT: i32 = 1;
    pub const ACTION_ATTACK: i32 = 2;
    pub const ACTION_CONSUME: i32 = 3;
    pub const ACTION_THROW: i32 = 4;
    pub const ACTION_SHOOT: i32 = 5;
    pub const ACTION_PLACE: i32 = 6;
    pub const ACTION_FILL_BOTTLE: i32 = 7;
    pub const ACTION_FILL_BUCKET: i32 = 8;
    pub const ACTION_POUR_BUCKET: i32 = 9;
    pub const ACTION_USE_TOOL: i32 = 10;
    pub const ACTION_INTERACT: i32 = 11;
    pub const ACTION_RETRIEVED: i32 = 12;
    pub const ACTION_DYED: i32 = 13;
    pub const ACTION_TRADED: i32 = 14;
    pub const ACTION_BRUSHING_COMPLETED: i32 = 15;
}
