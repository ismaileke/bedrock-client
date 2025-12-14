use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct SimulationType {
    pub simulation_type: u8,
}

pub fn new(simulation_type: u8) -> SimulationType {
    SimulationType { simulation_type }
}

impl Packet for SimulationType {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSimulationType.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.simulation_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SimulationType {
        let simulation_type = stream.get_byte();

        SimulationType { simulation_type }
    }

    fn debug(&self) {
        println!("Simulation Type: {}", self.simulation_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl SimulationType {
    pub const GAME: u8 = 0;
    pub const EDITOR: u8 = 1;
    pub const TEST: u8 = 2;
}
