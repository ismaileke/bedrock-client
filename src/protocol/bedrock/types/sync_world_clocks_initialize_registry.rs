use crate::protocol::bedrock::types::sync_world_clock_data::SyncWorldClockData;
use binary_utils::binary::{Reader, Writer};


#[derive(serde::Serialize, Debug)]
pub struct SyncWorldClocksInitializeRegistry {
    pub clock_data: Vec<SyncWorldClockData>
}

impl SyncWorldClocksInitializeRegistry {
    pub fn new(clock_data: Vec<SyncWorldClockData>) -> SyncWorldClocksInitializeRegistry {
        SyncWorldClocksInitializeRegistry { clock_data }
    }

    pub fn read(stream: &mut Reader) -> SyncWorldClocksInitializeRegistry {
        let len = stream.get_var_u32();
        let mut clock_data: Vec<SyncWorldClockData> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            clock_data.push(SyncWorldClockData::read(stream));
        }

        SyncWorldClocksInitializeRegistry { clock_data }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.clock_data.len() as u32);
        for clock_datum in &self.clock_data {
            clock_datum.write(stream);
        }
    }
}
