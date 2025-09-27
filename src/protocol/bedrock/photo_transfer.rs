use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct PhotoTransfer {
    pub photo_name: String,
    pub photo_data: String,
    pub book_id: String,
    pub photo_type: u8,
    pub source_type: u8,
    pub owner_actor_unique_id: i64,
    pub new_photo_name: String
}

pub fn new(photo_name: String, photo_data: String, book_id: String, photo_type: u8, source_type: u8, owner_actor_unique_id: i64, new_photo_name: String) -> PhotoTransfer {
    PhotoTransfer { photo_name, photo_data, book_id, photo_type, source_type, owner_actor_unique_id, new_photo_name }
}

impl Packet for PhotoTransfer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPhotoTransfer.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.photo_name.clone());
        PacketSerializer::put_string(&mut stream, self.photo_data.clone());
        PacketSerializer::put_string(&mut stream, self.book_id.clone());
        stream.put_byte(self.photo_type);
        stream.put_byte(self.source_type);
        stream.put_l_long(self.owner_actor_unique_id); //..?
        PacketSerializer::put_string(&mut stream, self.new_photo_name.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PhotoTransfer {
        let mut stream = Stream::new(bytes, 0);

        let photo_name = PacketSerializer::get_string(&mut stream);
        let photo_data = PacketSerializer::get_string(&mut stream);
        let book_id = PacketSerializer::get_string(&mut stream);
        let photo_type = stream.get_byte();
        let source_type = stream.get_byte();
        let owner_actor_unique_id = stream.get_l_long(); //..?
        let new_photo_name = PacketSerializer::get_string(&mut stream);

        PhotoTransfer { photo_name, photo_data, book_id, photo_type, source_type, owner_actor_unique_id, new_photo_name }
    }

    fn debug(&self) {
        println!("Photo Name: {}", self.photo_name);
        println!("Photo Data: {}", self.photo_data);
        println!("Book ID: {}", self.book_id);
        println!("Photo Type: {}", self.photo_type);
        println!("Source Type: {}", self.source_type);
        println!("Owner Actor Unique ID: {}", self.owner_actor_unique_id);
        println!("New Photo Name: {}", self.new_photo_name);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
