use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct ServerTelemetryData {
    pub server_id: String,
    pub scenario_id: String,
    pub world_id: String,
    pub owner_id: String
}

impl ServerTelemetryData {
    pub fn read(stream: &mut Stream) -> ServerTelemetryData {
        let server_id = PacketSerializer::get_string(stream);
        let scenario_id = PacketSerializer::get_string(stream);
        let world_id = PacketSerializer::get_string(stream);
        let owner_id = PacketSerializer::get_string(stream);

        ServerTelemetryData { server_id, scenario_id, world_id, owner_id }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.server_id.clone());
        PacketSerializer::put_string(stream, self.scenario_id.clone());
        PacketSerializer::put_string(stream, self.world_id.clone());
        PacketSerializer::put_string(stream, self.owner_id.clone());
    }
}