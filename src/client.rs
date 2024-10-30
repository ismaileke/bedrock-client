use crate::protocol::frame_set::{Datagram, Frame, FrameNumberCache, RELIABLE, RELIABLE_ORDERED, UNRELIABLE};
use crate::protocol::game::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::game::play_status::LoginStatus;
use crate::protocol::game::*;
use crate::protocol::game_packet::GamePacket;
use crate::protocol::packet_ids::{PacketType, MAGIC};
use crate::protocol::*;
use crate::utils::address::InternetAddress;
use crate::utils::color_format::COLOR_WHITE;
use crate::utils::encryption::Encryption;
use crate::utils::{address, color_format, encryption};
use crate::*;
use binary_utils::binary::Stream;
use chrono::Utc;
use minecraft_auth::bedrock;
use mojang_nbt::tag::compound_tag::CompoundTag;
use openssl::base64::decode_block;
use openssl::ec::EcKey;
use openssl::pkey::{PKey, Private};
use rand::Rng;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Result;
use std::net::UdpSocket;
use mojang_nbt::tag::tag::Tag;
use crate::protocol::acknowledge::Acknowledge;
use crate::protocol::conn_req::ConnReq;
use crate::protocol::conn_req_accepted::ConnReqAccepted;
use crate::protocol::connected_ping::ConnectedPing;
use crate::protocol::connected_pong::ConnectedPong;
use crate::protocol::new_incoming_conn::NewIncomingConn;
use crate::protocol::open_conn_reply1::OpenConnReply1;
use crate::protocol::open_conn_reply2::OpenConnReply2;
use crate::protocol::open_conn_req1::OpenConnReq1;
use crate::protocol::open_conn_req2::OpenConnReq2;
use crate::utils::chunk::block::{BlockType, PropertyValues};
//use crate::handle_incoming_data;


// conn_req update
// maybe encryption disabled on server? or xbox disabled? or compress disabled?
// if there is a skipped packet, wait for it, if you don't wait and try to decrypt it, you will get an 'invalid checksum' error
// NACK ACK System handler errors
// fragment packet receiving - sending etc.
// gönderdiğimiz paketleri buna kaydetme: FrameCache { //sequencenumber => framecache eğer nack gelirse birdaha göndeririz

pub struct Client {
    socket: UdpSocket,
    target_address: String,
    target_port: u16,
    client_guid: i64,
    chain: Vec<String>,
    ec_key: EcKey<Private>,
    game: GamePacket,
    frame_number_cache: FrameNumberCache,
    last_received_packets: HashMap<i32, Frame>, // reliable_frame_index: Frame
    last_received_fragment_packets: HashMap<u16, HashMap<u32, Vec<u8>>>, // split_id: index => buffer
    last_received_sequence_number: i32,
    last_handled_reliable_frame_index: i32,
    debug: bool,
    compression_enabled: bool,
    encryption_enabled: bool
}

pub async fn create(target_address: String, target_port: u16, client_version: String, debug: bool) -> Option<Client> {
    let mut bedrock = bedrock::new(client_version, false);
    if !bedrock.auth().await { return None; }
    let mut rng = rand::thread_rng();
    Option::from(Client{
        socket: UdpSocket::bind("0.0.0.0:0").expect("Socket Bind Error"),
        target_address,
        target_port,
        client_guid: rng.gen_range(10000..100000),
        chain: bedrock.get_chain_data(),
        ec_key: bedrock.get_ec_key()?,
        game: GamePacket::new(None, false),
        frame_number_cache: frame_set::start_number_cache(),
        last_received_packets: HashMap::new(),
        last_received_fragment_packets: HashMap::new(),
        last_received_sequence_number: -1,
        last_handled_reliable_frame_index: -1,
        debug,
        compression_enabled: false,
        encryption_enabled: false
    })
}

impl Client {
    pub fn connect(&mut self) -> Result<()> {
        if self.debug {
            println!("Local socket bound to: {}", self.socket.local_addr()?);
        }
        let address = format!("{}:{}", self.target_address, self.target_port);
        self.socket.connect(address)?;

        self.read_raknet_socket();

        Ok(())
    }

    fn read_raknet_socket(&mut self) {
        let req1: Vec<u8> = OpenConnReq1::new(MAGIC, RAKNET_PROTOCOL_VERSION, 1492).encode();
        self.socket.send(&req1).expect("Packet could not be sent");

        let mut buffer = vec![0; 2048];

        let mut should_stop = false;

        loop {
            if should_stop { break; }

            match self.socket.recv_from(&mut buffer) {
                Ok((amt, _src)) => {
                    let mut stream = Stream::new(Vec::from(&buffer[..amt]), 0);

                    let packet_id = stream.get_byte();
                    let packet_type = PacketType::from_byte(packet_id);

                    should_stop = self.raknet_packet_handler(packet_type, &mut stream);

                    if !frame_set::is_datagram(packet_id) { continue; }

                    let datagram = Datagram::from_binary(stream.get_buffer());

                    // SENDING ACK
                    let ack = Acknowledge::create(PacketType::ACK, 1, true, Option::from(datagram.sequence_number.clone()), None, None);
                    self.socket.send(&ack.encode()).expect("ACK Send Error");

                    let seq = datagram.sequence_number;

                    for frame in datagram.frames {
                        if let Some(reliable_frame_index) = frame.reliable_frame_index {
                            self.last_received_packets.insert(reliable_frame_index, frame);
                        } else {
                            // UNRELIABLE PACKET HANDLER
                            let mut stream = Stream::new(frame.body, 0);
                            let packet_id = stream.get_byte();
                            let packet_type = PacketType::from_byte(packet_id);

                            should_stop = self.raknet_packet_handler(packet_type, &mut stream);
                        }
                    }

                    // SENDING NACK
                    if (self.last_received_sequence_number + 1) != seq {
                        for seq_num in (self.last_received_sequence_number+1)..seq {
                            let nack = Acknowledge::create(PacketType::NACK, 1, true, Option::from(seq_num), None, None);
                            self.socket.send(&nack.encode()).expect("NACK Send Error");
                        }
                    }
                    if seq > self.last_received_sequence_number {
                        self.last_received_sequence_number = seq;
                    }

                    let mut sorted_reliable_frame_index: Vec<i32> = self.last_received_packets
                        .keys()
                        .cloned()
                        .collect();
                    sorted_reliable_frame_index.sort();

                    //fragment suspect
                    for reliable_frame_index in sorted_reliable_frame_index {
                        if reliable_frame_index <= self.last_handled_reliable_frame_index { //////////////////////////////////////////////////////////////////////////////
                            self.last_received_packets.remove(&reliable_frame_index);
                            continue;
                        }
                        if reliable_frame_index == self.last_handled_reliable_frame_index + 1 {
                            if let Some(frame) = self.last_received_packets.get(&reliable_frame_index) {
                                let mut real_body = frame.body.clone();

                                // FRAGMENT HANDLER
                                if let Some(fragment) = &frame.fragment {
                                    self.last_received_fragment_packets.entry(fragment.compound_id).or_insert_with(HashMap::new).insert(fragment.index, frame.body.clone());
                                    if let Some(fragment_data) = self.last_received_fragment_packets.get(&fragment.compound_id) {
                                        if (fragment_data.len() as u32) == fragment.compound_size {

                                            let mut keys: Vec<u32> = fragment_data.keys().cloned().collect();
                                            keys.sort();

                                            let mut result = Vec::new();
                                            for key in keys {
                                                if let Some(value) = fragment_data.get(&key) {
                                                    result.extend_from_slice(value);
                                                }
                                            }
                                            real_body = result;
                                        } else {
                                            self.last_handled_reliable_frame_index = reliable_frame_index;
                                            self.last_received_packets.remove(&reliable_frame_index);
                                            continue;
                                        }
                                    } else {
                                        self.last_handled_reliable_frame_index = reliable_frame_index;
                                        self.last_received_packets.remove(&reliable_frame_index);
                                        continue;
                                    }
                                }

                                // PACKET HANDLER
                                let mut stream = Stream::new(real_body, 0);
                                let packet_id = stream.get_byte();
                                let packet_type = PacketType::from_byte(packet_id);

                                match packet_type {
                                    PacketType::NACK => {
                                        let nack = Acknowledge::decode(stream.get_buffer());
                                        if self.debug { nack.debug(true); }
                                    }
                                    PacketType::ConnectedPing => {
                                        let connected_ping = ConnectedPing::decode(stream.get_buffer());
                                        if self.debug { connected_ping.debug(); }

                                        let connected_pong = ConnectedPong::create(connected_ping.ping_time, Utc::now().timestamp()).encode();
                                        let frame = Datagram::create_frame(connected_pong, UNRELIABLE, &self.frame_number_cache, None);
                                        let datagram = Datagram::create(vec![frame], &self.frame_number_cache).to_binary();
                                        self.frame_number_cache.sequence_number += 1;
                                        self.socket.send(&datagram).expect("ConnectedPong Packet could not be sent");
                                    },
                                    PacketType::ConnectedPong => {
                                        let connected_pong = ConnectedPong::decode(stream.get_buffer());
                                        if self.debug { connected_pong.debug(); }
                                        /*let connected_ping = connected_ping::create(Utc::now().timestamp()).encode();
                                        let frame = Datagram::create_frame(connected_ping, UNRELIABLE, &self.frame_number_cache, None);
                                        let datagram = Datagram::create(vec![frame], &self.frame_number_cache).to_binary();
                                        self.frame_number_cache.sequence_number += 1;
                                        self.socket.send(&datagram).expect("ConnectedPing Packet could not be sent");*/
                                    },
                                    PacketType::ConnReqAccepted => {
                                        self.raknet_packet_handler(PacketType::ConnReqAccepted, &mut stream);
                                    },
                                    PacketType::Game => {
                                        if self.encryption_enabled {
                                            stream = Stream::new(self.game.decrypt(&stream.get_remaining().unwrap()), 0);
                                        }

                                        if self.compression_enabled {
                                            let compression_type = stream.get_byte();

                                            println!("Compression Type: {}", if compression_type == 0 { format!("{}ZLIB{}", color_format::COLOR_AQUA, COLOR_WHITE) } else if compression_type == 1 { format!("{}SNAPPY{}", color_format::COLOR_AQUA, COLOR_WHITE) } else { format!("{}NONE{}", color_format::COLOR_AQUA, COLOR_WHITE) });
                                            if compression_type == 0 {
                                                stream = Stream::new(GamePacket::decompress(&stream.get_remaining().unwrap()), 0);
                                            }
                                        }

                                        while !stream.feof() {
                                            let length = stream.get_unsigned_var_int();

                                            let packet = stream.get(length).unwrap();
                                            let mut packet_stream = Stream::new(packet, 0);

                                            let packet_id = packet_stream.get_unsigned_var_int();
                                            let packet_type = BedrockPacketType::from_byte(packet_id as u16);

                                            println!("--- {}{}{} ---", color_format::COLOR_GOLD, BedrockPacketType::get_packet_name(packet_id as u16), COLOR_WHITE);
                                            match packet_type {
                                                BedrockPacketType::NetworkSettings => {
                                                    let network_settings = network_settings::decode(packet_stream.get_remaining().unwrap());
                                                    println!("Compression Threshold: {}", if network_settings.compression_threshold == 1 { "COMPRESS_EVERYTHING" } else { "COMPRESS_NOTHING" });
                                                    println!("Compression Algorithm: {}", if network_settings.compression_algorithm == 0 { "ZLIB" } else if network_settings.compression_algorithm == 1 { "SNAPPY" } else { "NONE" });
                                                    println!("Enable Client Throttling: {}", network_settings.enable_client_throttling);
                                                    println!("Client Throttle Threshold: {}", network_settings.client_throttle_threshold);
                                                    println!("Client Throttle Scalar: {}", network_settings.client_throttle_scalar);
                                                    self.compression_enabled = true;

                                                    // LOGIN PACKET
                                                    let pkey = PKey::from_ec_key(self.ec_key.clone()).expect("PKey Error");
                                                    let login_data_detail = login::convert_login_chain(&mut self.chain, pkey, self.target_address.clone(), self.target_port, self.client_guid);
                                                    let login = login::new(BEDROCK_PROTOCOL_VERSION, login_data_detail[0].clone(), login_data_detail[1].clone()).encode();

                                                    let datagrams = Datagram::split_packet(login, &mut self.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("Login Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::ServerToClientHandshake => {
                                                    let s_to_c_handshake = server_to_client_handshake::decode(packet_stream.get_remaining().unwrap());
                                                    let jwt = String::from_utf8(s_to_c_handshake.jwt).unwrap();
                                                    println!("JWT: {:?}", jwt);
                                                    let jwt_split: Vec<&str> = jwt.split('.').collect();

                                                    let jwt_header = Encryption::b64_url_decode(jwt_split[0]).unwrap();
                                                    let jwt_header_value: Value = serde_json::from_str(jwt_header.as_str()).expect("JWT Header can not decoded.");

                                                    let jwt_payload = Encryption::b64_url_decode(jwt_split[1]).unwrap();
                                                    let jwt_payload_value: Value = serde_json::from_str(jwt_payload.as_str()).expect("JWT Payload can not decoded.");

                                                    let x5u = jwt_header_value.get("x5u").and_then(Value::as_str).unwrap().to_string();
                                                    let server_private = encryption::parse_der_public_key(decode_block(x5u.as_str()).unwrap().as_slice());
                                                    let salt = decode_block(jwt_payload_value.get("salt").and_then(Value::as_str).unwrap()).unwrap();

                                                    let local_pkey = PKey::from_ec_key(self.ec_key.clone()).expect("Local PKey Error");
                                                    let shared_secret = encryption::generate_shared_secret(local_pkey, server_private);
                                                    let encryption_key = encryption::generate_key(&shared_secret, salt);
                                                    let encryption = Encryption::fake_gcm(encryption_key).expect("Encryption Fake GCM Error");

                                                    self.game = GamePacket::new(Option::from(encryption), self.compression_enabled);
                                                    self.encryption_enabled = true;

                                                    // CLIENT TO SERVER HANDSHAKE PACKET
                                                    let c_to_s_handshake = client_to_server_handshake::new().encode();

                                                    let game_packet = self.game.encode(&c_to_s_handshake);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("ClientToServerHandshake Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::ResourcePacksInfo => {
                                                    let resource_packs_info = resource_packs_info::decode(packet_stream.get_remaining().unwrap());
                                                    let mut rp_uuids = Vec::new();
                                                    println!("Must Accept: {}", resource_packs_info.must_accept);
                                                    println!("Has Addons: {}", resource_packs_info.has_addons);
                                                    println!("Has Scripts: {}", resource_packs_info.has_scripts);
                                                    let resource_pack_count = resource_packs_info.resource_packs.len();
                                                    println!("Resource Pack Count: {}", resource_pack_count);
                                                    for (i, resource_pack) in resource_packs_info.resource_packs.iter().enumerate() {
                                                        rp_uuids.push(resource_pack.uuid.clone());
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

                                                    // RESOURCE PACK CLIENT RESPONSE PACKET {COMPLETED}
                                                    let rp_client_response = resource_pack_client_response::new(resource_pack_client_response::COMPLETED, rp_uuids).encode();

                                                    let game_packet = self.game.encode(&rp_client_response);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("ResourcePackClientResponse Packet Fragment could not be sent");
                                                    }

                                                    // CLIENT CACHE STATUS PACKET
                                                    let client_cache_status = client_cache_status::new(false).encode();

                                                    let game_packet = self.game.encode(&client_cache_status);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("ClientCacheStatus Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::PlayStatus => {
                                                    let play_status = play_status::decode(packet_stream.get_remaining().unwrap());
                                                    let status = LoginStatus::try_from(play_status.status).unwrap();
                                                    if play_status.status == 3 { // Player Spawn
                                                        // SET LOCAL PLAYER AS INITIALIZED PACKET
                                                        let set_local_player_as_init = set_local_player_as_initialized::new(0).encode();

                                                        let game_packet = self.game.encode(&set_local_player_as_init);

                                                        let datagrams = Datagram::split_packet(game_packet, &mut self.frame_number_cache);

                                                        for datagram in datagrams {
                                                            self.socket.send(&datagram.to_binary()).expect("SetLocalPlayerAsInitialized Packet Fragment could not be sent");
                                                        }
                                                    }
                                                    match status {
                                                        LoginStatus::LoginSuccess => println!("Status: {}Login Success{}", color_format::COLOR_GREEN, COLOR_WHITE),
                                                        LoginStatus::LoginFailedClient => println!("Status: {}Login Failed Client{}", color_format::COLOR_RED, COLOR_WHITE),
                                                        LoginStatus::LoginFailedServer => println!("Status: {}Login Failed Server{}", color_format::COLOR_RED, COLOR_WHITE),
                                                        LoginStatus::PlayerSpawn => println!("Status: {}Player Spawn{}", color_format::COLOR_GREEN, COLOR_WHITE),
                                                        LoginStatus::LoginFailedInvalidTenant => println!("Status: {}Login Failed Invalid Tenant{}", color_format::COLOR_RED, COLOR_WHITE),
                                                        LoginStatus::LoginFailedVanillaEdu => println!("Status: {}Login Failed Vanilla Edu{}", color_format::COLOR_RED, COLOR_WHITE),
                                                        LoginStatus::LoginFailedEduVanilla => println!("Status: {}Login Failed Edu Vanilla{}", color_format::COLOR_RED, COLOR_WHITE),
                                                        LoginStatus::LoginFailedServerFull => println!("Status: {}Login Failed Server Full{}", color_format::COLOR_RED, COLOR_WHITE),
                                                        LoginStatus::LoginFailedEditorVanilla => println!("Status: {}Login Failed Editor Vanilla{}", color_format::COLOR_RED, COLOR_WHITE),
                                                        LoginStatus::LoginFailedVanillaEditor => println!("Status: {}Login Failed Vanilla Editor{}", color_format::COLOR_RED, COLOR_WHITE),
                                                    }
                                                },
                                                BedrockPacketType::StartGame => {
                                                    let start_game = start_game::decode(packet_stream.get_remaining().unwrap());

                                                    println!("actor_unique_id: {}", start_game.actor_unique_id);
                                                    println!("actor_runtime_id: {}", start_game.actor_runtime_id);
                                                    println!("server_software_version: {}", start_game.server_software_version);
                                                    println!("player_game_mode: {}", start_game.player_game_mode);
                                                    println!("player_position: {:?}", start_game.player_position);
                                                    println!("yaw: {}", start_game.yaw);
                                                    println!("pitch: {}", start_game.pitch);
                                                    println!("level_settings: {:?}", start_game.level_settings);
                                                    println!("level_id: {}", start_game.level_id);
                                                    println!("world_name: {}", start_game.world_name);
                                                    println!("premium_world_template_id: {}", start_game.premium_world_template_id);
                                                    println!("is_trial: {}", start_game.is_trial);
                                                    println!("player_movement_settings: {:?}", start_game.player_movement_settings);
                                                    println!("current_tick: {}", start_game.current_tick);
                                                    println!("enchantment_seed: {}", start_game.enchantment_seed);

                                                    let block_palette = start_game.block_palette;
                                                    for block in &block_palette {

                                                        let mut block_type: BlockType = BlockType::new(block.get_name());
                                                        let mut block_properties: HashMap<String, PropertyValues> = HashMap::new();

                                                        let root = block.get_states().get_root();
                                                        let bct = root.as_any().downcast_ref::<CompoundTag>().unwrap();
                                                        /*for (key, value) in bct.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
                                                            println!(" - {} - {}", key, value.get_type());
                                                        }
                                                        if start_game.block_network_ids_are_hashes {} else {}*/

                                                        let properties = bct.get_list_tag("properties".to_string());
                                                        if let Some(data) = properties {
                                                            for value in data.get_value().downcast_ref::<Vec<Box<dyn Tag>>>().unwrap() {
                                                                let c_tag = value.as_any().downcast_ref::<CompoundTag>().unwrap();

                                                                let property_name = c_tag.get_string("name").unwrap();
                                                                let list_enum = c_tag.get_list_tag("enum".to_string()).unwrap();

                                                                let mut strings = Vec::new();
                                                                let mut ints = Vec::new();
                                                                let mut bools = Vec::new();
                                                                for value in list_enum.get_value().downcast_ref::<Vec<Box<dyn Tag>>>().unwrap() {
                                                                    let inner_value = value.get_value();
                                                                    if let Some(v) = inner_value.downcast_ref::<String>() {
                                                                        strings.push(v.to_string());
                                                                    } else if let Some(v) = inner_value.downcast_ref::<u32>() {
                                                                        ints.push(v.clone());
                                                                    } else if let Some(v) = inner_value.downcast_ref::<bool>() {
                                                                        bools.push(v.clone());
                                                                    } else {
                                                                        println!("enum: value - Unknown type");
                                                                    }
                                                                }
                                                                let enums = PropertyValues{ strings, bools, ints };

                                                                block_properties.insert(property_name, enums);
                                                                block_type.properties = block_properties.clone();

                                                            }
                                                        }

                                                    }

                                                    println!("multiplayer_correlation_id: {}", start_game.multiplayer_correlation_id);
                                                    println!("enable_new_inventory_system: {}", start_game.enable_new_inventory_system);
                                                    println!("server_software_version: {}", start_game.server_software_version);
                                                    //println!("player_actor_properties: {}", start_game.player_actor_properties);
                                                    println!("block_palette_checksum: {:?}", start_game.block_palette_checksum);
                                                    println!("world_template_id: {:?}", start_game.world_template_id);
                                                    println!("enable_client_side_chunk_generation: {}", start_game.enable_client_side_chunk_generation);
                                                    println!("block_network_ids_are_hashes: {}", start_game.block_network_ids_are_hashes);
                                                    println!("network_permissions: {:?}", start_game.network_permissions);
                                                    /*let item_table = start_game.item_table;
                                                    for item in &item_table {
                                                        println!("-----\nstring_id: {}", item.get_string_id());
                                                        println!("numeric_id: {}", item.get_numeric_id());
                                                        println!("component_based: {}", item.is_component_based());
                                                    }*/

                                                },
                                                BedrockPacketType::AvailableCommands => {
                                                    // REQUEST CHUNK RADIUS PACKET
                                                    let req_chunk_radius = request_chunk_radius::new(40, 40).encode();

                                                    let game_packet = self.game.encode(&req_chunk_radius);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("RequestChunkRadius Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::Text => {
                                                    let text = text::decode(packet_stream.get_remaining().unwrap());
                                                    if let Some(source_name) = text.source_name {
                                                        println!("Source Name: {}", source_name);
                                                    }
                                                    println!("Message: {}", text.message);
                                                    if let Some(parameters) = text.parameters {
                                                        println!("Parameters: {}", parameters.join(" "));
                                                        //handle_incoming_data(parameters.join(" ").into_bytes());
                                                    } else {
                                                        //handle_incoming_data(text.message.into_bytes());
                                                    }

                                                },
                                                BedrockPacketType::LevelChunk => {
                                                    let level_chunk = level_chunk::decode(packet_stream.get_remaining().unwrap());
                                                    println!("Chunk X: {}", level_chunk.chunk_x);
                                                    println!("Chunk Z: {}", level_chunk.chunk_z);
                                                    println!("Dimension ID: {}", level_chunk.dimension_id);
                                                    println!("Sub Chunk Count: {}", level_chunk.sub_chunk_count);
                                                    println!("Client Sub Chunk Requests Enabled: {}", level_chunk.client_sub_chunk_requests_enabled);
                                                    println!("Used Blob Hashes: {:?}", level_chunk.used_blob_hashes);
                                                    println!("Extra Payload: {:?}", level_chunk.extra_payload.len());
                                                    //ChunkResolve::new(level_chunk);
                                                }
                                                BedrockPacketType::Disconnect => {
                                                    let disconnect = disconnect::decode(packet_stream.get_remaining().unwrap());
                                                    println!("Reason: {}", disconnect.reason);
                                                    if !disconnect.skip_message {
                                                        println!("Message: {}", disconnect.message.unwrap());
                                                        println!("Filtered Message: {}", disconnect.filtered_message.unwrap());
                                                    }
                                                    should_stop = true;
                                                }
                                                _ => {}
                                            }
                                        }
                                    },
                                    PacketType::DisconnectionNotification => {
                                        println!("{}Disconnect Notification Packet Received{}", color_format::COLOR_RED, COLOR_WHITE);
                                        should_stop = true;
                                    }
                                    _ => {}
                                }
                                self.last_handled_reliable_frame_index = reliable_frame_index;
                                self.last_received_packets.remove(&reliable_frame_index);
                            }
                        }
                    }

                }
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                }
            }
        }
    }

    fn raknet_packet_handler(&mut self, packet_type: PacketType, stream: &mut Stream) -> bool {
        let mut should_stop = false;

        match packet_type {
            PacketType::OpenConnReply1 => {
                let open_conn_reply1 = OpenConnReply1::decode(stream.get_buffer());
                if self.debug { open_conn_reply1.debug(); }

                let req2 = OpenConnReq2::new(MAGIC, address::new(4, self.target_address.to_string(), self.target_port), open_conn_reply1.cookie, false, open_conn_reply1.mtu, self.client_guid).encode();
                self.socket.send(&req2).expect("Open Connection Request 2 Packet could not be sent");
            },
            PacketType::OpenConnReply2 => {
                let open_conn_reply2 = OpenConnReply2::decode(stream.get_buffer());
                if self.debug { open_conn_reply2.debug(); }

                let body = ConnReq::new(self.client_guid, Utc::now().timestamp(), false).encode();

                let frame = Datagram::create_frame(body, RELIABLE, &self.frame_number_cache, None);
                let datagram = Datagram::create(vec![frame], &self.frame_number_cache).to_binary();
                self.frame_number_cache.sequence_number += 1;
                self.frame_number_cache.reliable_frame_index += 1;

                self.socket.send(&datagram).expect("Connection Request Packet could not be sent");
            },
            PacketType::ConnReqAccepted => {

                let conn_req_accepted = ConnReqAccepted::decode(stream.get_buffer());
                if self.debug { conn_req_accepted.debug(); }

                // New Incoming Connection
                let addresses: [InternetAddress; 20] = core::array::from_fn(|_| address::new(4, "0.0.0.0".to_string(), 0));
                let new_incoming_conn = NewIncomingConn::new(address::new(4, self.target_address.to_string(), self.target_port), addresses, Utc::now().timestamp(), Utc::now().timestamp() + 1).encode();
                let frame = Datagram::create_frame(new_incoming_conn, RELIABLE_ORDERED, &self.frame_number_cache, None);
                self.frame_number_cache.reliable_frame_index += 1;
                self.frame_number_cache.ordered_frame_index += 1;

                // Connected Ping
                let connected_ping = ConnectedPing::create(Utc::now().timestamp()).encode();
                let frame_two = Datagram::create_frame(connected_ping, UNRELIABLE, &self.frame_number_cache, None);

                // Request Network Settings Packet
                let request_network_settings = req_network_settings::new(BEDROCK_PROTOCOL_VERSION).encode();
                let frame_three = Datagram::create_frame(request_network_settings, RELIABLE_ORDERED, &self.frame_number_cache, None);

                let datagram = Datagram::create(vec![frame, frame_two, frame_three], &self.frame_number_cache).to_binary();
                self.frame_number_cache.sequence_number += 1;
                self.frame_number_cache.reliable_frame_index += 1;
                self.frame_number_cache.ordered_frame_index += 1;

                self.socket.send(&datagram).expect("NewIncomingConnection & RequestNetworkSettings Packet could not be sent");
                //should_stop = true;
            },
            PacketType::IncompatibleProtocol => {
                let incompatible_protocol = incompatible_protocol::decode(stream.get_buffer());
                println!("{}Incompatible Protocol Version, Server Protocol Version: {}{}", color_format::COLOR_RED, incompatible_protocol.server_protocol, COLOR_WHITE);
                should_stop = true;
            },
            PacketType::DisconnectionNotification => {
                println!("{}Disconnection Notification Packet Received From Server.{}", color_format::COLOR_RED, COLOR_WHITE);
                should_stop = true;
            },
            _ => { /*vec![]*/ }
        };

        should_stop
    }
}
