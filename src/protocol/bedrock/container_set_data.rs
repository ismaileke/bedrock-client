use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ContainerSetData {
    pub window_id: u8,
    pub property: i32,
    pub value: i32,
}

impl Packet for ContainerSetData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDContainerSetData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.window_id);
        stream.put_var_i32(self.property);
        stream.put_var_i32(self.value);
    }

    fn decode(stream: &mut Reader) -> ContainerSetData {
        let window_id = stream.get_u8();
        let property = stream.get_var_i32();
        let value = stream.get_var_i32();

        ContainerSetData { window_id, property, value }
    }


}

impl ContainerSetData {
    pub const PROPERTY_FURNACE_SMELT_PROGRESS: i32 = 0;
    pub const PROPERTY_FURNACE_REMAINING_FUEL_TIME: i32 = 1;
    pub const PROPERTY_FURNACE_MAX_FUEL_TIME: i32 = 2;
    pub const PROPERTY_FURNACE_STORED_XP: i32 = 3;
    pub const PROPERTY_FURNACE_FUEL_AUX: i32 = 4;

    pub const PROPERTY_BREWING_STAND_BREW_TIME: i32 = 0;
    pub const PROPERTY_BREWING_STAND_FUEL_AMOUNT: i32 = 1;
    pub const PROPERTY_BREWING_STAND_FUEL_TOTAL: i32 = 2;
}
