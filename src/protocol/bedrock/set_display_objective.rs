use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

pub struct SetDisplayObjective {
    pub display_slot: String,
    pub objective_name: String,
    pub display_name: String,
    pub criteria_name: String,
    pub sort_order: i32
}

pub fn new(display_slot: String, objective_name: String, display_name: String, criteria_name: String, sort_order: i32) -> SetDisplayObjective {
    SetDisplayObjective { display_slot, objective_name, display_name, criteria_name, sort_order }
}

impl SetDisplayObjective {
    pub const DISPLAY_SLOT_LIST: &'static str = "list";
    pub const DISPLAY_SLOT_SIDEBAR: &'static str = "sidebar";
    pub const DISPLAY_SLOT_BELOW_NAME: &'static str = "belowname";

    pub const SORT_ORDER_ASCENDING: i32 = 0;
    pub const SORT_ORDER_DESCENDING: i32 = 1;
    
}

impl Packet for SetDisplayObjective {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetDisplayObjective.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.display_slot.clone());
        PacketSerializer::put_string(&mut stream, self.objective_name.clone());
        PacketSerializer::put_string(&mut stream, self.display_name.clone());
        PacketSerializer::put_string(&mut stream, self.criteria_name.clone());
        stream.put_var_int(self.sort_order);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetDisplayObjective {
        let mut stream = Stream::new(bytes, 0);

        let display_slot = PacketSerializer::get_string(&mut stream);
        let objective_name = PacketSerializer::get_string(&mut stream);
        let display_name = PacketSerializer::get_string(&mut stream);
        let criteria_name = PacketSerializer::get_string(&mut stream);
        let sort_order = stream.get_var_int();

        SetDisplayObjective { display_slot, objective_name, display_name, criteria_name, sort_order }
    }

    fn debug(&self) {
        println!("Display Slot: {}", self.display_slot);
        println!("Objective Name: {}", self.objective_name);
        println!("Display Name: {}", self.display_name);
        println!("Criteria Name: {}", self.criteria_name);
        println!("Sort Order: {}", self.sort_order);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
