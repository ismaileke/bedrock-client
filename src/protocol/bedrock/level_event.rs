use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct LevelEvent {
    pub event_id: i32, //see types/level_event
    pub position: Vec<f32>,
    pub event_data: i32
}

pub fn new(event_id: i32, position: Vec<f32>, event_data: i32) -> LevelEvent {
    LevelEvent { event_id, position, event_data }
}

impl Packet for LevelEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLevelEvent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.event_id);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        stream.put_var_int(self.event_data);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> LevelEvent {
        let mut stream = Stream::new(bytes, 0);

        let event_id = stream.get_var_int();
        let position = PacketSerializer::get_vector3(&mut stream);
        let event_data = stream.get_var_int();

        LevelEvent { event_id, position, event_data }
    }

    fn debug(&self) {
        println!("Event ID: {}", self.event_id);
        println!("Position: {:?}", self.position);
        println!("Event Data: {}", self.event_data);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
