use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct CorrectPlayerMovePrediction {
    pub prediction_type: u8,
    pub position: Vec<f32>,
    pub delta: Vec<f32>,
    pub vehicle_rotation_x: f32,
    pub vehicle_rotation_y: f32,
    pub vehicle_angular_velocity: Option<f32>,
    pub on_ground: bool,
    pub tick: u64,
}

impl CorrectPlayerMovePrediction {
    pub const PREDICTION_TYPE_PLAYER: u8 = 0;
    pub const PREDICTION_TYPE_VEHICLE: u8 = 1;
}

impl Packet for CorrectPlayerMovePrediction {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCorrectPlayerMovePrediction.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.prediction_type);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        PacketSerializer::put_vector3(&mut stream, self.delta.clone());
        stream.put_f32_le(self.vehicle_rotation_x);
        stream.put_f32_le(self.vehicle_rotation_y);
        PacketSerializer::write_optional(&mut stream, &self.vehicle_angular_velocity, |s, v| {
            s.put_f32_le(*v)
        });
        stream.put_bool(self.on_ground);
        stream.put_var_u64(self.tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CorrectPlayerMovePrediction {
        let prediction_type = stream.get_byte();
        let position = PacketSerializer::get_vector3(stream);
        let delta = PacketSerializer::get_vector3(stream);
        let vehicle_rotation_x = stream.get_f32_le();
        let vehicle_rotation_y = stream.get_f32_le();
        let vehicle_angular_velocity = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let on_ground = stream.get_bool();
        let tick = stream.get_var_u64();

        CorrectPlayerMovePrediction {
            prediction_type,
            position,
            delta,
            vehicle_rotation_x,
            vehicle_rotation_y,
            vehicle_angular_velocity,
            on_ground,
            tick,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
