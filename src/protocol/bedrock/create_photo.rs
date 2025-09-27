use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct CreatePhoto {
    pub actor_unique_id: i64,
    pub photo_name: String,
    pub photo_item_name: String
}

pub fn new(actor_unique_id: i64, photo_name: String, photo_item_name: String) -> CreatePhoto {
    CreatePhoto { actor_unique_id, photo_name, photo_item_name }
}

impl Packet for CreatePhoto {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCreatePhoto.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_long(self.actor_unique_id); // WHY??
        PacketSerializer::put_string(&mut stream, self.photo_name.clone());
        PacketSerializer::put_string(&mut stream, self.photo_item_name.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CreatePhoto {
        let mut stream = Stream::new(bytes, 0);

        let actor_unique_id = stream.get_l_long();
        let photo_name = PacketSerializer::get_string(&mut stream);
        let photo_item_name = PacketSerializer::get_string(&mut stream);

        CreatePhoto { actor_unique_id, photo_name, photo_item_name }
    }

    fn debug(&self) {
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Photo Name: {}", self.photo_name);
        println!("Photo Item Name: {}", self.photo_item_name);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
