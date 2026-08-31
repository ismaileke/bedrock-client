use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct EntityDiagnosticTimingInfo {
    pub display_name: String,
    pub entity: String,
    pub time_in_ns: u64,
    pub percent_of_total: u8,
}

impl EntityDiagnosticTimingInfo {
    pub fn new(display_name: String, entity: String, time_in_ns: u64, percent_of_total: u8) -> EntityDiagnosticTimingInfo {
        EntityDiagnosticTimingInfo { display_name, entity, time_in_ns, percent_of_total }
    }

    pub fn read(stream: &mut Reader) -> EntityDiagnosticTimingInfo {
        let display_name = PacketSerializer::get_string(stream);
        let entity = PacketSerializer::get_string(stream);
        let time_in_ns = stream.get_u64_le();
        let percent_of_total = stream.get_u8();
        EntityDiagnosticTimingInfo { display_name, entity, time_in_ns, percent_of_total }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.display_name);
        PacketSerializer::put_string(stream, &self.entity);
        stream.put_u64_le(self.time_in_ns);
        stream.put_u8(self.percent_of_total);
    }
}
