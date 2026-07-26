use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::locator_bar_waypoint_payload::LocatorBarWaypointPayload;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct LocatorBar {
    pub way_points: Vec<LocatorBarWaypointPayload>
}

impl Packet for LocatorBar {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLocatorBar.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.way_points.len() as u32);
        for way_point in &self.way_points {
            way_point.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> LocatorBar {
        let len = stream.get_var_u32() as usize;
        let mut way_points = Vec::with_capacity(len);
        for _ in 0..len {
            way_points.push(LocatorBarWaypointPayload::read(stream));
        }
        
        LocatorBar { way_points }
    }
}
