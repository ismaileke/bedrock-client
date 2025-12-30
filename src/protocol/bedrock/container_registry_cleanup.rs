use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ContainerRegistryCleanup {
    pub removed_containers: Vec<FullContainerName>,
}

impl Packet for ContainerRegistryCleanup {
    fn id(&self) -> u16 {
        BedrockPacketType::IDContainerRegistryCleanup.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.removed_containers.len() as u32);
        for container in self.removed_containers.iter() {
            container.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ContainerRegistryCleanup {
        let removed_containers_count = stream.get_var_u32() as usize;
        let mut removed_containers = Vec::new();
        for _ in 0..removed_containers_count {
            removed_containers.push(FullContainerName::read(stream));
        }

        ContainerRegistryCleanup { removed_containers }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
