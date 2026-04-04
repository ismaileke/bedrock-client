use crate::protocol::bedrock::types::sync_world_clock_marker_data::SyncWorldClockMarkerData;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SyncWorldClocksRemoveTimeMarker {
    pub clock_id: u64,
    pub markers: Vec<SyncWorldClockMarkerData>
}

impl SyncWorldClocksRemoveTimeMarker {
    pub fn new(clock_id: u64, markers: Vec<SyncWorldClockMarkerData>) -> SyncWorldClocksRemoveTimeMarker {
        SyncWorldClocksRemoveTimeMarker  { clock_id, markers }
    }

    pub fn read(stream: &mut Stream) -> SyncWorldClocksRemoveTimeMarker {
        let clock_id = stream.get_var_u64();
        let len = stream.get_var_u32();
        let mut markers: Vec<SyncWorldClockMarkerData> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            markers.push(SyncWorldClockMarkerData::read(stream));
        }

        SyncWorldClocksRemoveTimeMarker { clock_id, markers }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u64(self.clock_id);
        stream.put_var_u32(self.markers.len() as u32);
        for marker in &self.markers {
            marker.write(stream);
        }
    }
}
