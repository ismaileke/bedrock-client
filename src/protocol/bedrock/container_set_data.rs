use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct ContainerSetData {
    pub window_id: u8,
    pub property: i32,
    pub value: i32
}

pub fn new(window_id: u8, property: i32, value: i32) -> ContainerSetData {
    ContainerSetData { window_id, property, value }
}

impl Packet for ContainerSetData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDContainerSetData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.window_id);
        stream.put_var_i32(self.property);
        stream.put_var_i32(self.value);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ContainerSetData {
        let window_id = stream.get_byte();
        let property = stream.get_var_i32();
        let value = stream.get_var_i32();

        ContainerSetData { window_id, property, value }
    }

    fn debug(&self) {
        println!("Window ID: {}", self.window_id);
        println!("Property: {}", self.property);
        println!("Value: {}", self.value);
    }

    fn as_any(&self) -> &dyn Any {
        self
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
