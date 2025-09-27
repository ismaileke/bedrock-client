use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct CorrectPlayerMovePrediction {
    pub prediction_type: u8,
    pub position: Vec<f32>,
    pub delta: Vec<f32>,
    pub vehicle_rotation_x: f32,
    pub vehicle_rotation_y: f32,
    pub vehicle_angular_velocity: Option<f32>,
    pub on_ground: bool,
    pub tick: u64
}

pub fn new(prediction_type: u8, position: Vec<f32>, delta: Vec<f32>, vehicle_rotation_x: f32, vehicle_rotation_y: f32, vehicle_angular_velocity: Option<f32>, on_ground: bool, tick: u64) -> CorrectPlayerMovePrediction {
    CorrectPlayerMovePrediction { prediction_type, position, delta, vehicle_rotation_x, vehicle_rotation_y, vehicle_angular_velocity, on_ground, tick }
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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_byte(self.prediction_type);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        PacketSerializer::put_vector3(&mut stream, self.delta.clone());
        stream.put_float(self.vehicle_rotation_x);
        stream.put_float(self.vehicle_rotation_y);
        PacketSerializer::write_optional(&mut stream, &self.vehicle_angular_velocity, |s, v| s.put_float(*v));
        stream.put_bool(self.on_ground);
        stream.put_unsigned_var_long(self.tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CorrectPlayerMovePrediction {
        let mut stream = Stream::new(bytes, 0);

        let prediction_type = stream.get_byte();
        let position = PacketSerializer::get_vector3(&mut stream);
        let delta = PacketSerializer::get_vector3(&mut stream);
        let vehicle_rotation_x = stream.get_float();
        let vehicle_rotation_y = stream.get_float();
        let vehicle_angular_velocity = PacketSerializer::read_optional(&mut stream, |s| s.get_float());
        let on_ground = stream.get_bool();
        let tick = stream.get_unsigned_var_long();

        CorrectPlayerMovePrediction { prediction_type, position, delta, vehicle_rotation_x, vehicle_rotation_y, vehicle_angular_velocity, on_ground, tick }
    }

    fn debug(&self) {
        println!("Prediction Type: {}", self.prediction_type);
        println!("Position: {:?}", self.position);
        println!("Delta: {:?}", self.delta);
        println!("Vehicle Rotation X: {}", self.vehicle_rotation_x);
        println!("Vehicle Rotation Y: {}", self.vehicle_rotation_y);
        println!("Vehicle Angular Velocity: {:?}", self.vehicle_angular_velocity);
        println!("On Ground: {}", self.on_ground);
        println!("Tick: {}", self.tick);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
