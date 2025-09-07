use std::collections::HashMap;
use minecraft_auth::bedrock::Bedrock;
use mojang_nbt::tag::compound_tag::CompoundTag;
use openssl::ec::EcKey;
use openssl::pkey::Private;

pub struct BedrockPacketHandler {
    pub chain: Vec<String>,
    pub ec_key: EcKey<Private>,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub hashed_network_ids: HashMap<u32, CompoundTag>,
    pub runtime_network_ids: Vec<CompoundTag>,
    pub air_network_id: u32
}

impl BedrockPacketHandler {
    pub fn new(bedrock: Bedrock) -> BedrockPacketHandler {
        let chain = bedrock.get_chain_data();
        let ec_key = bedrock.get_ec_key().unwrap();
        let compression_enabled = false;
        let encryption_enabled = false;
        let hashed_network_ids = HashMap::new();
        let runtime_network_ids = vec![];
        let air_network_id = 0;

        BedrockPacketHandler{
            chain,
            ec_key,
            compression_enabled,
            encryption_enabled,
            hashed_network_ids,
            runtime_network_ids,
            air_network_id,
        }
    }
}