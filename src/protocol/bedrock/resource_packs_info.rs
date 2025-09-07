use binary_utils::binary::Stream;
use uuid::Uuid;

pub struct ResourcePacksInfo {
    pub must_accept: bool,
    pub has_addons: bool,
    pub has_scripts: bool,
    pub world_template_id: String,
    pub world_template_version: String,
    pub force_disable_vibrant_visuals: bool,
    pub resource_packs: Vec<ResourcePack>
}

pub struct ResourcePack {
    pub uuid: String,
    pub version: String,
    pub size_bytes: i64,
    pub encryption_key: String,
    pub sub_pack_name: String,
    pub content_id: String,
    pub has_scripts: bool,
    pub is_addon_pack: bool,
    pub is_rtx_capable: bool,
    pub cdn_url: String
}

impl ResourcePacksInfo {
    pub fn debug(&self) {
        println!("Must Accept: {}", self.must_accept);
        println!("Has Addons: {}", self.has_addons);
        println!("Has Scripts: {}", self.has_scripts);
        println!("World Template ID: {}", self.world_template_id);
        println!("World Template Version: {}", self.world_template_version);
        println!("Force Disable Vibrant Visuals: {}", self.force_disable_vibrant_visuals);
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
}

pub fn decode(bytes: Vec<u8>) -> ResourcePacksInfo {
    let mut stream = Stream::new(bytes, 0);

    let must_accept = stream.get_bool();
    let has_addons = stream.get_bool();
    let has_scripts = stream.get_bool();
    let force_disable_vibrant_visuals = stream.get_bool();

    let world_template_id = Uuid::from_slice(&stream.get(16).unwrap()).unwrap().to_string();
    let length = stream.get_unsigned_var_int();
    let world_template_version = String::from_utf8(stream.get(length).unwrap()).unwrap();

    let resource_pack_count = stream.get_l_short();
    let mut resource_packs = Vec::new();
    for _ in 0..resource_pack_count {
        let uuid = Uuid::from_slice(&stream.get(16).unwrap()).unwrap().to_string();
        let mut length = stream.get_unsigned_var_int();
        let version = String::from_utf8(stream.get(length).unwrap()).unwrap();
        let size_bytes = stream.get_l_long();
        length = stream.get_unsigned_var_int();
        let encryption_key = String::from_utf8(stream.get(length).unwrap()).unwrap();
        length = stream.get_unsigned_var_int();
        let sub_pack_name = String::from_utf8(stream.get(length).unwrap()).unwrap();
        length = stream.get_unsigned_var_int();
        let content_id = String::from_utf8(stream.get(length).unwrap()).unwrap();
        let has_scripts = stream.get_bool();
        let is_addon_pack = stream.get_bool();
        let is_rtx_capable = stream.get_bool();
        length = stream.get_unsigned_var_int();
        let cdn_url = String::from_utf8(stream.get(length).unwrap()).unwrap();

        resource_packs.push(ResourcePack{ uuid, version, size_bytes, encryption_key, sub_pack_name, content_id, has_scripts, is_addon_pack, is_rtx_capable, cdn_url });
    }

    ResourcePacksInfo { must_accept, has_addons, has_scripts, world_template_id, world_template_version, force_disable_vibrant_visuals, resource_packs }
}