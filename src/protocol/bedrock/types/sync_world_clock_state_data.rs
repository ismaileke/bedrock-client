use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SyncWorldClockStateData {
    pub clock_id: u64,
    pub time: i32,
    pub paused: bool
}

impl SyncWorldClockStateData {
    pub fn new(clock_id: u64, time: i32, paused: bool) -> SyncWorldClockStateData {
        SyncWorldClockStateData { clock_id, time, paused }
    }

    pub fn read(stream: &mut Stream) -> SyncWorldClockStateData {
        let clock_id = stream.get_var_u64();
        let time = stream.get_var_i32();
        let paused = stream.get_bool();

        SyncWorldClockStateData { clock_id, time, paused }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u64(self.clock_id);
        stream.put_i32_le(self.time);
        stream.put_bool(self.paused);
    }
}
