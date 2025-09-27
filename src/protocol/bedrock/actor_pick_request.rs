use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ActorPickRequest {
    pub actor_unique_id: i64,
    pub add_user_data: bool,
    pub hotbar_slot: u8
}

pub fn new(actor_unique_id: i64, add_user_data: bool, hotbar_slot: u8) -> ActorPickRequest {
    ActorPickRequest { actor_unique_id, add_user_data, hotbar_slot }
}

impl Packet for ActorPickRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDActorPickRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        stream.put_bool(self.add_user_data);
        stream.put_byte(self.hotbar_slot);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ActorPickRequest {
        let mut stream = Stream::new(bytes, 0);

        let actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);
        let add_user_data = stream.get_bool();
        let hotbar_slot = stream.get_byte();

        ActorPickRequest { actor_unique_id, add_user_data, hotbar_slot }
    }

    fn debug(&self) {
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Add User Data: {}", self.add_user_data);
        println!("Hotbar Slot: {}", self.hotbar_slot);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
