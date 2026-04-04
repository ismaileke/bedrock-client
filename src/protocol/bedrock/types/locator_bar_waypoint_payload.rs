use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::locator_bar_waypoint::LocatorBarWaypoint;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct LocatorBarWaypointPayload {
    pub group: String,
    pub waypoint: LocatorBarWaypoint,
    pub action: u8,
}

impl LocatorBarWaypointPayload {
    pub fn new(group: String, waypoint: LocatorBarWaypoint, action: u8) -> LocatorBarWaypointPayload {
        LocatorBarWaypointPayload { group, waypoint, action }
    }

    pub fn read(stream: &mut Stream) -> LocatorBarWaypointPayload {
        let group = PacketSerializer::get_uuid(stream);
        let waypoint = LocatorBarWaypoint::read(stream);
        let action = stream.get_byte();

        LocatorBarWaypointPayload { group, waypoint, action }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_uuid(stream, self.group.clone());
        self.waypoint.write(stream);
        stream.put_byte(self.action);
    }
}
