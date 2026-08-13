use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerFog {
    pub fog_layers: Vec<String>,
}

impl Packet for PlayerFog {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerFog.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.fog_layers.len() as u32);
        for fog_layer in self.fog_layers.iter() {
            PacketSerializer::put_string(stream, fog_layer.clone());
        }
    }

    fn decode(stream: &mut Reader) -> PlayerFog {
        let fog_layers_len = stream.get_var_u32() as usize;
        let mut fog_layers = Vec::new();
        for _ in 0..fog_layers_len {
            fog_layers.push(PacketSerializer::get_string(stream));
        }

        PlayerFog { fog_layers }
    }
}
