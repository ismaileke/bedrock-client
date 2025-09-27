use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;

pub struct ContainerRegistryCleanup {
    pub removed_containers: Vec<FullContainerName>
}

pub fn new(removed_containers: Vec<FullContainerName>) -> ContainerRegistryCleanup {
    ContainerRegistryCleanup { removed_containers }
}

impl Packet for ContainerRegistryCleanup {
    fn id(&self) -> u16 {
        BedrockPacketType::IDContainerRegistryCleanup.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.removed_containers.len() as u32);
        for container in self.removed_containers.iter() {
            container.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ContainerRegistryCleanup {
        let mut stream = Stream::new(bytes, 0);

        let removed_containers_count = stream.get_unsigned_var_int() as usize;
        let mut removed_containers = Vec::new();
        for _ in 0..removed_containers_count {
            removed_containers.push(FullContainerName::read(&mut stream));
        }

        ContainerRegistryCleanup { removed_containers }
    }

    fn debug(&self) {
        println!("Removed Containers: {:?}", self.removed_containers);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
