use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SimulationType {
    pub simulation_type: u8,
}

impl Packet for SimulationType {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSimulationType.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.simulation_type);
    }

    fn decode(stream: &mut Reader) -> SimulationType {
        let simulation_type = stream.get_u8();

        SimulationType { simulation_type }
    }
}

impl SimulationType {
    pub const GAME: u8 = 0;
    pub const EDITOR: u8 = 1;
    pub const TEST: u8 = 2;
}
