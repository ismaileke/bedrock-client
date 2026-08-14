use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DataStoreRemoval {
    pub name: String,
}

impl DataStoreRemoval {
    pub fn new(name: String) -> DataStoreRemoval {
        DataStoreRemoval { name }
    }

    pub fn read(stream: &mut Reader) -> DataStoreRemoval {
        DataStoreRemoval {
            name: PacketSerializer::get_string(stream),
        }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.name);
    }
}
