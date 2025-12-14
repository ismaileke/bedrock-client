use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct CreatePhoto {
    pub actor_unique_id: i64,
    pub photo_name: String,
    pub photo_item_name: String,
}

pub fn new(actor_unique_id: i64, photo_name: String, photo_item_name: String) -> CreatePhoto {
    CreatePhoto {
        actor_unique_id,
        photo_name,
        photo_item_name,
    }
}

impl Packet for CreatePhoto {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCreatePhoto.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_i64_le(self.actor_unique_id);
        PacketSerializer::put_string(&mut stream, self.photo_name.clone());
        PacketSerializer::put_string(&mut stream, self.photo_item_name.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CreatePhoto {
        let actor_unique_id = stream.get_i64_le();
        let photo_name = PacketSerializer::get_string(stream);
        let photo_item_name = PacketSerializer::get_string(stream);

        CreatePhoto {
            actor_unique_id,
            photo_name,
            photo_item_name,
        }
    }

    fn debug(&self) {
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Photo Name: {}", self.photo_name);
        println!("Photo Item Name: {}", self.photo_item_name);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
