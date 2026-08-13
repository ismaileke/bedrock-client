use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetDisplayObjective {
    pub display_slot: String,
    pub objective_name: String,
    pub display_name: String,
    pub criteria_name: String,
    pub sort_order: i32,
}

impl Packet for SetDisplayObjective {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetDisplayObjective.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.display_slot.clone());
        PacketSerializer::put_string(stream, self.objective_name.clone());
        PacketSerializer::put_string(stream, self.display_name.clone());
        PacketSerializer::put_string(stream, self.criteria_name.clone());
        stream.put_var_i32(self.sort_order);
    }

    fn decode(stream: &mut Reader) -> SetDisplayObjective {
        let display_slot = PacketSerializer::get_string(stream);
        let objective_name = PacketSerializer::get_string(stream);
        let display_name = PacketSerializer::get_string(stream);
        let criteria_name = PacketSerializer::get_string(stream);
        let sort_order = stream.get_var_i32();

        SetDisplayObjective {
            display_slot,
            objective_name,
            display_name,
            criteria_name,
            sort_order,
        }
    }
}

impl SetDisplayObjective {
    pub const DISPLAY_SLOT_LIST: &'static str = "list";
    pub const DISPLAY_SLOT_SIDEBAR: &'static str = "sidebar";
    pub const DISPLAY_SLOT_BELOW_NAME: &'static str = "belowname";

    pub const SORT_ORDER_ASCENDING: i32 = 0;
    pub const SORT_ORDER_DESCENDING: i32 = 1;
}
