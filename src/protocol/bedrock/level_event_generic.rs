use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use mojang_nbt::base_nbt_serializer::BaseNBTSerializer;
use mojang_nbt::nbt::TAG_COMPOUND;
use mojang_nbt::tag::tag::Tag;
use crate::protocol::bedrock::serializer::network_nbt_serializer::NetworkNBTSerializer;

pub struct LevelEventGeneric {
    pub event_id: i32,
    pub event_data: Box<dyn Tag>
}

pub fn new(event_id: i32, event_data: Box<dyn Tag>) -> LevelEventGeneric {
    LevelEventGeneric { event_id, event_data }
}

impl Packet for LevelEventGeneric {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLevelEventGeneric.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.event_id);
        let mut nbt_serializer = NetworkNBTSerializer::new();
        let data = nbt_serializer.write_headless(self.event_data.clone());
        stream.put(data);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> LevelEventGeneric {
        let event_id = stream.get_var_i32();
        let mut offset = stream.get_offset();
        let mut nbt_serializer = NetworkNBTSerializer::new();
        let event_data = nbt_serializer.read_headless(Vec::from(stream.get_buffer()), &mut offset, TAG_COMPOUND, 0);
        stream.set_offset(offset);

        LevelEventGeneric { event_id, event_data }
    }

    fn debug(&self) {
        println!("Event ID: {}", self.event_id);
        println!("Event Data: {:?}", self.event_data.get_value());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
