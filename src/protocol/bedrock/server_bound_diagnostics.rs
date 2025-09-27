use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct ServerBoundDiagnostics {
    pub avg_fps: f32,
    pub avg_server_sim_tick_time_ms: f32,
    pub avg_client_sim_tick_time_ms: f32,
    pub avg_begin_frame_time_ms: f32,
    pub avg_input_time_ms: f32,
    pub avg_render_time_ms: f32,
    pub avg_end_frame_time_ms: f32,
    pub avg_remainder_time_percent: f32,
    pub avg_unaccounted_time_percent: f32
}

pub fn new(avg_fps: f32,
           avg_server_sim_tick_time_ms: f32,
           avg_client_sim_tick_time_ms: f32,
           avg_begin_frame_time_ms: f32,
           avg_input_time_ms: f32,
           avg_render_time_ms: f32,
           avg_end_frame_time_ms: f32,
           avg_remainder_time_percent: f32,
           avg_unaccounted_time_percent: f32) -> ServerBoundDiagnostics {
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
    }
}

impl Packet for ServerBoundDiagnostics {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundDiagnostics.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_float(self.avg_fps);
        stream.put_l_float(self.avg_server_sim_tick_time_ms);
        stream.put_l_float(self.avg_client_sim_tick_time_ms);
        stream.put_l_float(self.avg_begin_frame_time_ms);
        stream.put_l_float(self.avg_input_time_ms);
        stream.put_l_float(self.avg_render_time_ms);
        stream.put_l_float(self.avg_end_frame_time_ms);
        stream.put_l_float(self.avg_remainder_time_percent);
        stream.put_l_float(self.avg_unaccounted_time_percent);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ServerBoundDiagnostics {
        let mut stream = Stream::new(bytes, 0);

        let avg_fps = stream.get_l_float();
        let avg_server_sim_tick_time_ms = stream.get_l_float();
        let avg_client_sim_tick_time_ms = stream.get_l_float();
        let avg_begin_frame_time_ms = stream.get_l_float();
        let avg_input_time_ms = stream.get_l_float();
        let avg_render_time_ms = stream.get_l_float();
        let avg_end_frame_time_ms = stream.get_l_float();
        let avg_remainder_time_percent = stream.get_l_float();
        let avg_unaccounted_time_percent = stream.get_l_float();

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
        }
    }

    fn debug(&self) {
        println!("Avg FPS: {}", self.avg_fps);
        println!("Avg Server Sim Tick Time MS: {}", self.avg_server_sim_tick_time_ms);
        println!("Avg Client Sim Tick Time MS: {}", self.avg_client_sim_tick_time_ms);
        println!("Avg Begin Frame Time MS: {}", self.avg_begin_frame_time_ms);
        println!("Avg Input Time MS: {}", self.avg_input_time_ms);
        println!("Avg Render Time MS: {}", self.avg_render_time_ms);
        println!("Avg End Frame Time MS: {}", self.avg_end_frame_time_ms);
        println!("Avg Remainder Time Percent: {}", self.avg_remainder_time_percent);
        println!("Avg Unaccounted Time Percent: {}", self.avg_unaccounted_time_percent);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
