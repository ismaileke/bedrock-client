use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct SimulationType {
    pub simulation_type: u8
}

pub fn new(simulation_type: u8) -> SimulationType {
    SimulationType { simulation_type }
}

impl SimulationType {
    pub const GAME: u8 = 0;
    pub const EDITOR: u8 = 1;
    pub const TEST: u8 = 2;
}

impl Packet for SimulationType {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSimulationType.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_byte(self.simulation_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SimulationType {
        let mut stream = Stream::new(bytes, 0);

        let simulation_type = stream.get_byte();

        SimulationType { simulation_type }
    }

    fn debug(&self) {
        println!("Simulation Type: {}", self.simulation_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
