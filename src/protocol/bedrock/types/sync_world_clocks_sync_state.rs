use crate::protocol::bedrock::types::sync_world_clock_state_data::SyncWorldClockStateData;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SyncWorldClocksSyncState {
    pub clock_data: Vec<SyncWorldClockStateData>
}

impl SyncWorldClocksSyncState {
    pub fn new(clock_data: Vec<SyncWorldClockStateData>) -> SyncWorldClocksSyncState {
        SyncWorldClocksSyncState  { clock_data }
    }

    pub fn read(stream: &mut Stream) -> SyncWorldClocksSyncState {
        let len = stream.get_var_u32();
        let mut clock_data: Vec<SyncWorldClockStateData> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            clock_data.push(SyncWorldClockStateData::read(stream));
        }

        SyncWorldClocksSyncState { clock_data }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.clock_data.len() as u32);
        for clock in &self.clock_data {
            clock.write(stream);
        }
    }
}
