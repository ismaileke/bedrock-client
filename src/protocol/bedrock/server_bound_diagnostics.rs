use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;
use crate::protocol::bedrock::types::memory_category_counter::MemoryCategoryCounter;

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
    pub memory_category_values: Vec<MemoryCategoryCounter>
}

impl Packet for ServerBoundDiagnostics {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundDiagnostics.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

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
            memory_category_value.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ServerBoundDiagnostics {
        let avg_fps = stream.get_f32_le();
        let avg_server_sim_tick_time_ms = stream.get_f32_le();
        let avg_client_sim_tick_time_ms = stream.get_f32_le();
        let avg_begin_frame_time_ms = stream.get_f32_le();
        let avg_input_time_ms = stream.get_f32_le();
        let avg_render_time_ms = stream.get_f32_le();
        let avg_end_frame_time_ms = stream.get_f32_le();
        let avg_remainder_time_percent = stream.get_f32_le();
        let avg_unaccounted_time_percent = stream.get_f32_le();
        let count = stream.get_var_u32();
        let mut memory_category_values = Vec::new();
        for _ in 0..count {
            memory_category_values.push(MemoryCategoryCounter::read(stream));
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
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
