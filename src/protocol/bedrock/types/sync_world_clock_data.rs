use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::sync_world_clock_marker_data::SyncWorldClockMarkerData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SyncWorldClockData {
    pub id: u64,
    pub name: String,
    pub time: i32,
    pub paused: bool,
    pub markers: Vec<SyncWorldClockMarkerData>,
}

impl SyncWorldClockData {
    pub fn new(id: u64, name: String, time: i32, paused: bool, markers: Vec<SyncWorldClockMarkerData>) -> SyncWorldClockData {
        SyncWorldClockData { id, name, time, paused, markers }
    }

    pub fn read(stream: &mut Reader) -> SyncWorldClockData {
        let id = stream.get_var_u64();
        let name = PacketSerializer::get_string(stream);
        let time = stream.get_var_i32();
        let paused = stream.get_bool();
        let len = stream.get_var_u32();
        let mut markers = Vec::with_capacity(len as usize);
        for _ in 0..len {
            markers.push(SyncWorldClockMarkerData::read(stream));
        }

        SyncWorldClockData { id, name, time, paused, markers }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u64(self.id);
        PacketSerializer::put_string(stream, &self.name);
        stream.put_var_i32(self.time);
        stream.put_bool(self.paused);
        stream.put_var_u32(self.markers.len() as u32);
        for marker in &self.markers {
            marker.write(stream);
        }
    }
}
