use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::memory_category_counter::MemoryCategoryCounter;
use crate::protocol::bedrock::types::entity_diagnostics_timing_info::EntityDiagnosticTimingInfo;
use crate::protocol::bedrock::types::system_diagnostics_timing_info::SystemDiagnosticTimingInfo;
use crate::protocol::bedrock::types::whisker_scope_data_summary::WhiskerScopeDataSummary;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundDiagnostics {
    pub avg_fps: f32,
    pub avg_server_sim_tick_time_ms: f32,
    pub avg_client_sim_tick_time_ms: f32,
    pub avg_begin_frame_time_ms: f32,
    pub avg_input_time_ms: f32,
    pub avg_render_time_ms: f32,
    pub avg_end_frame_time_ms: f32,
    pub avg_remainder_time_percent: f32,
    pub avg_unaccounted_time_percent: f32,
    pub memory_category_values: Vec<MemoryCategoryCounter>,
    pub entity_diagnostics: Vec<EntityDiagnosticTimingInfo>,
    pub system_diagnostics: Vec<SystemDiagnosticTimingInfo>,
    pub whisker_scopes: Vec<WhiskerScopeDataSummary>
}

impl Packet for ServerBoundDiagnostics {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundDiagnostics.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_f32_le(self.avg_fps);
        stream.put_f32_le(self.avg_server_sim_tick_time_ms);
        stream.put_f32_le(self.avg_client_sim_tick_time_ms);
        stream.put_f32_le(self.avg_begin_frame_time_ms);
        stream.put_f32_le(self.avg_input_time_ms);
        stream.put_f32_le(self.avg_render_time_ms);
        stream.put_f32_le(self.avg_end_frame_time_ms);
        stream.put_f32_le(self.avg_remainder_time_percent);
        stream.put_f32_le(self.avg_unaccounted_time_percent);
        stream.put_var_u32(self.memory_category_values.len() as u32);
        for memory_category_value in &self.memory_category_values {
            memory_category_value.write(stream);
        }
        stream.put_var_u32(self.entity_diagnostics.len() as u32);
        for entity_diagnostics_value in &self.entity_diagnostics {
            entity_diagnostics_value.write(stream);
        }
        stream.put_var_u32(self.system_diagnostics.len() as u32);
        for system_diagnostics_value in &self.system_diagnostics {
            system_diagnostics_value.write(stream);
        }
        stream.put_var_u32(self.whisker_scopes.len() as u32);
        for whisker_scopes_value in &self.whisker_scopes {
            whisker_scopes_value.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> ServerBoundDiagnostics {
        let avg_fps = stream.get_f32_le();
        let avg_server_sim_tick_time_ms = stream.get_f32_le();
        let avg_client_sim_tick_time_ms = stream.get_f32_le();
        let avg_begin_frame_time_ms = stream.get_f32_le();
        let avg_input_time_ms = stream.get_f32_le();
        let avg_render_time_ms = stream.get_f32_le();
        let avg_end_frame_time_ms = stream.get_f32_le();
        let avg_remainder_time_percent = stream.get_f32_le();
        let avg_unaccounted_time_percent = stream.get_f32_le();
        let mut count = stream.get_var_u32();
        let mut memory_category_values = Vec::new();
        for _ in 0..count {
            memory_category_values.push(MemoryCategoryCounter::read(stream));
        }
        count = stream.get_var_u32();
        let mut entity_diagnostics = Vec::new();
        for _ in 0..count {
            entity_diagnostics.push(EntityDiagnosticTimingInfo::read(stream));
        }
        count = stream.get_var_u32();
        let mut system_diagnostics = Vec::new();
        for _ in 0..count {
            system_diagnostics.push(SystemDiagnosticTimingInfo::read(stream));
        }
        count = stream.get_var_u32();
        let mut whisker_scopes = Vec::new();
        for _ in 0..count {
            whisker_scopes.push(WhiskerScopeDataSummary::read(stream));
        }

        ServerBoundDiagnostics {
            avg_fps,
            avg_server_sim_tick_time_ms,
            avg_client_sim_tick_time_ms,
            avg_begin_frame_time_ms,
            avg_input_time_ms,
            avg_render_time_ms,
            avg_end_frame_time_ms,
            avg_remainder_time_percent,
            avg_unaccounted_time_percent,
            memory_category_values,
            entity_diagnostics,
            system_diagnostics,
            whisker_scopes
        }
    }
}
