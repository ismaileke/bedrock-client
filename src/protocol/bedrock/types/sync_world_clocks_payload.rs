use crate::protocol::bedrock::types::sync_world_clocks_add_time_marker::SyncWorldClocksAddTimeMarker;
use crate::protocol::bedrock::types::sync_world_clocks_initialize_registry::SyncWorldClocksInitializeRegistry;
use crate::protocol::bedrock::types::sync_world_clocks_remove_time_marker::SyncWorldClocksRemoveTimeMarker;
use crate::protocol::bedrock::types::sync_world_clocks_sync_state::SyncWorldClocksSyncState;
use binary_utils::binary::Stream;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum SyncWorldClocksPayload {
    SyncState(SyncWorldClocksSyncState),
    InitializeRegistry(SyncWorldClocksInitializeRegistry),
    AddTimeMarker(SyncWorldClocksAddTimeMarker),
    RemoveTimeMarker(SyncWorldClocksRemoveTimeMarker),
}

impl SyncWorldClocksPayload {
    pub const SYNC_STATE: u32 = 0;
    pub const INITIALIZE_REGISTRY: u32 = 1;
    pub const ADD_TIME_MARKER: u32 = 2;
    pub const REMOVE_TIME_MARKER: u32 = 3;

    pub fn id(&self) -> u32 {
        match self {
            SyncWorldClocksPayload::SyncState(_) => Self::SYNC_STATE,
            SyncWorldClocksPayload::InitializeRegistry(_) => Self::INITIALIZE_REGISTRY,
            SyncWorldClocksPayload::AddTimeMarker(_) => Self::ADD_TIME_MARKER,
            SyncWorldClocksPayload::RemoveTimeMarker(_) => Self::REMOVE_TIME_MARKER,
        }
    }

    pub fn read(stream: &mut Stream) -> SyncWorldClocksPayload {
        let payload_type = stream.get_var_u32();
        match payload_type {
            SyncWorldClocksPayload::SYNC_STATE => SyncWorldClocksPayload::SyncState(SyncWorldClocksSyncState::read(stream)),
            SyncWorldClocksPayload::INITIALIZE_REGISTRY => SyncWorldClocksPayload::InitializeRegistry(SyncWorldClocksInitializeRegistry::read(stream)),
            SyncWorldClocksPayload::ADD_TIME_MARKER => SyncWorldClocksPayload::AddTimeMarker(SyncWorldClocksAddTimeMarker::read(stream)),
            SyncWorldClocksPayload::REMOVE_TIME_MARKER => SyncWorldClocksPayload::RemoveTimeMarker(SyncWorldClocksRemoveTimeMarker::read(stream)),
            _ => panic!("Sync world clocks payload type not handled: {}", payload_type),
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        match self {
            SyncWorldClocksPayload::SyncState(r) => r.write(stream),
            SyncWorldClocksPayload::InitializeRegistry(r) => r.write(stream),
            SyncWorldClocksPayload::AddTimeMarker(r) => r.write(stream),
            SyncWorldClocksPayload::RemoveTimeMarker(r) => r.write(stream),
        }
    }
}
