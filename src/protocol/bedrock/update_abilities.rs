use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::abilities_data::AbilitiesData;

pub struct UpdateAbilities {
    pub data: AbilitiesData
}

pub fn new(data: AbilitiesData) -> UpdateAbilities {
    UpdateAbilities { data }
}

impl Packet for UpdateAbilities {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateAbilities.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        self.data.write(&mut stream);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> UpdateAbilities {
        let mut stream = Stream::new(bytes, 0);

        let data = AbilitiesData::read(&mut stream);

        UpdateAbilities { data }
    }

    fn debug(&self) {
        println!("Ability Data: {:?}", self.data);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
