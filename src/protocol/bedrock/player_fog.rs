use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct PlayerFog {
    pub fog_layers: Vec<String>
}

pub fn new(fog_layers: Vec<String>) -> PlayerFog {
    PlayerFog { fog_layers }
}

impl Packet for PlayerFog {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerFog.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.fog_layers.len() as u32);
        for fog_layer in self.fog_layers.iter() {
            PacketSerializer::put_string(&mut stream, fog_layer.clone());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> PlayerFog {
        let mut stream = Stream::new(bytes, 0);

        let fog_layers_len = stream.get_var_u32() as usize;
        let mut fog_layers = Vec::new();
        for _ in 0..fog_layers_len {
            fog_layers.push(PacketSerializer::get_string(&mut stream));
        }

        PlayerFog { fog_layers }
    }

    fn debug(&self) {
        println!("Fog Layers: {:?}", self.fog_layers);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
