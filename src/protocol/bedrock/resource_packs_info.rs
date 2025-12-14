use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ResourcePacksInfo {
    pub must_accept: bool,
    pub has_addons: bool,
    pub has_scripts: bool,
    pub world_template_id: String,
    pub world_template_version: String,
    pub force_disable_vibrant_visuals: bool,
    pub resource_packs: Vec<ResourcePack>,
}

#[derive(serde::Serialize, Debug)]
pub struct ResourcePack {
    pub uuid: String,
    pub version: String,
    pub size_bytes: u64,
    pub encryption_key: String,
    pub sub_pack_name: String,
    pub content_id: String,
    pub has_scripts: bool,
    pub is_addon_pack: bool,
    pub is_rtx_capable: bool,
    pub cdn_url: String,
}

impl Packet for ResourcePacksInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePacksInfo.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        todo!()
    }

    fn decode(stream: &mut Stream) -> ResourcePacksInfo {
        let must_accept = stream.get_bool();
        let has_addons = stream.get_bool();
        let has_scripts = stream.get_bool();
        let force_disable_vibrant_visuals = stream.get_bool();
        let world_template_id = PacketSerializer::get_uuid(stream);
        let world_template_version = PacketSerializer::get_string(stream);

        let resource_pack_count = stream.get_u16_le();
        let mut resource_packs = Vec::new();
        for _ in 0..resource_pack_count {
            let uuid = PacketSerializer::get_uuid(stream);
            let version = PacketSerializer::get_string(stream);
            let size_bytes = stream.get_u64_le();
            let encryption_key = PacketSerializer::get_string(stream);
            let sub_pack_name = PacketSerializer::get_string(stream);
            let content_id = PacketSerializer::get_string(stream);
            let has_scripts = stream.get_bool();
            let is_addon_pack = stream.get_bool();
            let is_rtx_capable = stream.get_bool();
            let cdn_url = PacketSerializer::get_string(stream);

            resource_packs.push(ResourcePack {
                uuid,
                version,
                size_bytes,
                encryption_key,
                sub_pack_name,
                content_id,
                has_scripts,
                is_addon_pack,
                is_rtx_capable,
                cdn_url,
            });
        }

        ResourcePacksInfo {
            must_accept,
            has_addons,
            has_scripts,
            world_template_id,
            world_template_version,
            force_disable_vibrant_visuals,
            resource_packs,
        }
    }

    fn debug(&self) {
        println!("Must Accept: {}", self.must_accept);
        println!("Has Addons: {}", self.has_addons);
        println!("Has Scripts: {}", self.has_scripts);
        println!("World Template ID: {}", self.world_template_id);
        println!("World Template Version: {}", self.world_template_version);
        println!(
            "Force Disable Vibrant Visuals: {}",
            self.force_disable_vibrant_visuals
        );
        println!("Resource Pack Count: {}", self.resource_packs.len());
        for (i, resource_pack) in self.resource_packs.iter().enumerate() {
            println!("- Resource Pack {} -", i + 1);
            println!(" - UUID: {}", resource_pack.uuid);
            println!(" - Version: {}", resource_pack.version);
            println!(" - Size Bytes: {}", resource_pack.size_bytes);
            println!(" - Encryption Key: {}", resource_pack.encryption_key);
            println!(" - Sub Pack Name: {}", resource_pack.sub_pack_name);
            println!(" - Content ID: {}", resource_pack.content_id);
            println!(" - Has Scripts: {}", resource_pack.has_scripts);
            println!(" - Is Addon Pack: {}", resource_pack.is_addon_pack);
            println!(" - Is RTX Capable: {}", resource_pack.is_rtx_capable);
            println!(" - CDN URL: {}", resource_pack.cdn_url);
            println!("-------------------");
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
