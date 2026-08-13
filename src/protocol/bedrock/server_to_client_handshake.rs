use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerToClientHandshake {
    pub jwt: Vec<u8>,
}

impl Packet for ServerToClientHandshake {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerToClientHandshake.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.jwt.len() as u32);
        stream.put(&self.jwt);
    }

    fn decode(stream: &mut Reader) -> ServerToClientHandshake {
        //let jwt = PacketSerializer::get_string(stream);
        let length = stream.get_var_u32();
        let jwt = stream.get(length as usize).to_vec();

        ServerToClientHandshake { jwt }
    }
}
