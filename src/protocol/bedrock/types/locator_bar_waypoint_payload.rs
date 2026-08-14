use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::locator_bar_waypoint::LocatorBarWaypoint;
use binary_utils::binary::{Reader, Writer};

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

    pub fn read(stream: &mut Reader) -> LocatorBarWaypointPayload {
        let group = PacketSerializer::get_uuid(stream);
        let waypoint = LocatorBarWaypoint::read(stream);
        let action = stream.get_u8();

        LocatorBarWaypointPayload { group, waypoint, action }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_uuid(stream, &self.group);
        self.waypoint.write(stream);
        stream.put_u8(self.action);
    }
}
