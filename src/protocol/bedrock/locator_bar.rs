use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::locator_bar_waypoint_payload::LocatorBarWaypointPayload;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct LocatorBar {
    pub way_points: Vec<LocatorBarWaypointPayload>
}

impl Packet for LocatorBar {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLocatorBar.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.way_points.len() as u32);
        for way_point in &self.way_points {
            way_point.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> LocatorBar {
        let len = stream.get_var_u32() as usize;
        let mut way_points = Vec::with_capacity(len);
        for _ in 0..len {
            way_points.push(LocatorBarWaypointPayload::read(stream));
        }
        
        LocatorBar { way_points }
    }
}
