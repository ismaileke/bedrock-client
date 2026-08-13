use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ContainerRegistryCleanup {
    pub removed_containers: Vec<FullContainerName>,
}

impl Packet for ContainerRegistryCleanup {
    fn id(&self) -> u16 {
        BedrockPacketType::IDContainerRegistryCleanup.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.removed_containers.len() as u32);
        for container in self.removed_containers.iter() {
            container.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> ContainerRegistryCleanup {
        let removed_containers_count = stream.get_var_u32() as usize;
        let mut removed_containers = Vec::new();
        for _ in 0..removed_containers_count {
            removed_containers.push(FullContainerName::read(stream));
        }

        ContainerRegistryCleanup { removed_containers }
    }
}
