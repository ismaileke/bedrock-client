use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SyncWorldClockMarkerData {
    pub id: u64,
    pub name: String,
    pub time: i32,
    pub period: Option<i32>
}

impl SyncWorldClockMarkerData {
    pub fn new(id: u64, name: String, time: i32, period: Option<i32>) -> SyncWorldClockMarkerData {
        SyncWorldClockMarkerData { id, name, time, period }
    }

    pub fn read(stream: &mut Reader) -> SyncWorldClockMarkerData {
        let id = stream.get_var_u64();
        let name = PacketSerializer::get_string(stream);
        let time = stream.get_var_i32();
        let period = PacketSerializer::read_optional(stream, |s| s.get_i32_le());

        SyncWorldClockMarkerData { id, name, time, period }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u64(self.id);
        PacketSerializer::put_string(stream, &self.name);
        stream.put_var_i32(self.time);
        PacketSerializer::write_optional(stream, &self.period, |s, v| s.put_i32_le(*v));
    }
}
