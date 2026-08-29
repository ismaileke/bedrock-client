use crate::handler::bedrock_packet_handler::BedrockPacketHandler;
use crate::handler::raknet_packet_handler::RakNetPacketHandler;
use crate::protocol::bedrock::bedrock_packet_ids::{BedrockPacket, BedrockPacketType};
use crate::protocol::bedrock::client_cache_status::ClientCacheStatus;
use crate::protocol::bedrock::client_to_server_handshake::ClientToServerHandshake;
use crate::protocol::bedrock::login::Login;
use crate::protocol::bedrock::network_stack_latency::NetworkStackLatency;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::request_chunk_radius::RequestChunkRadius;
use crate::protocol::bedrock::resource_pack_client_response::ResourcePackClientResponse;
use crate::protocol::bedrock::set_local_player_as_initialized::SetLocalPlayerAsInitializedPacket;
use crate::protocol::bedrock::*;
use crate::protocol::raknet::acknowledge::Acknowledge;
use crate::protocol::raknet::connected_ping::ConnectedPing;
use crate::protocol::raknet::connected_pong::ConnectedPong;
use crate::protocol::raknet::frame_set;
use crate::protocol::raknet::frame_set::{Datagram, UNRELIABLE};
use crate::protocol::raknet::game_packet::GamePacket;
use crate::protocol::raknet::open_conn_req1::OpenConnReq1;
use crate::protocol::raknet::packet_ids::{PacketType, MAGIC};
use crate::utils::block::PropertyValue;
use crate::utils::chunk::Chunk;
use crate::utils::color_format::*;
use crate::utils::encryption::Encryption;
use crate::utils::{block, encryption};
use crate::*;
use base64::engine::general_purpose;
use base64::Engine;
use chrono::Utc;
use flate2::read::GzDecoder;
use linked_hash_map::LinkedHashMap;
use minecraft_auth::bedrock;
use mojang_nbt::nbt::NBT;
use mojang_nbt::tag::compound_tag::CompoundTag;
use mojang_nbt::tag::tag::Tag;
use mojang_nbt::tree_root::TreeRoot;
use mojang_nbt::nbt_serializer::{NBTReader, NBTWriter};
use serde_json::Value;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::io;
use binary_utils::binary::{Reader, Writer};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use crate::protocol::bedrock::types::block_palette_entry::BlockPaletteEntry;

pub struct Client {
    // Network
    network_sender: UnboundedSender<Box<dyn Packet>>, // Game -> Network
    network_receiver: UnboundedReceiver<ClientEvent>, // Network -> Game
    pub target_address: String,
    pub target_port: u16,
    pub client_version: String,
    pub debug: bool,
    pub auth_callback: Arc<Mutex<Option<Box<dyn Fn(&str, &str) + Send>>>>,
    // In Game
    pub chunk_palette_hashed: HashMap<u32, CompoundTag>,
    pub chunk_palette_runtime: Vec<CompoundTag>,
    pub chunk_air_id: u32,
    pub runtime_id: u64,
    pub unique_id: i64,
    pub current_tick: u64,
    pub player_position: Vec<f32>,
    pub yaw: f32,
    pub pitch: f32,
}

pub enum ClientEvent {
    Packet(String, BedrockPacket),
    GameStarted {
        hashed_ids: HashMap<u32, CompoundTag>,
        runtime_ids: Vec<CompoundTag>,
        air_id: u32,
        runtime_id: u64,
        unique_id: i64,
        current_tick: u64,
        player_position: Vec<f32>,
        yaw: f32,
        pitch: f32,
    }
}

pub async fn create<F>(
    target_address: String,
    target_port: u16,
    client_version: String,
    debug: bool,
    auth_callback_fn: F
) -> Option<Client>
where
    F: Fn(&str, &str) + Send + 'static
{
    let mut address = String::new();

    match lookup_host(&target_address) {
        Ok(addrs) => {
            for addr in addrs {
                address = addr.ip().to_string();
                break;
            }
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    }

    let auth_callback: Arc<Mutex<Option<Box<dyn Fn(&str, &str) + Send>>>> = Arc::new(Mutex::new(Some(Box::new(auth_callback_fn))));
    let auth_callback_clone = auth_callback.clone();

    let mut bedrock = bedrock::new(client_version.clone(), false);
    bedrock.set_auth_callback(move |code, url| {
        if let Some(callback) = &*auth_callback_clone.lock().unwrap() {
            callback(code, url);
        }
    });
    bedrock.auth().await;

    // (Queue System)
    let (tx_outbound, rx_outbound) = unbounded_channel::<Box<dyn Packet>>(); // From game to network
    let (tx_inbound, rx_inbound) = unbounded_channel::<ClientEvent>(); // From the Network to the Game

    let raknet_handler = RakNetPacketHandler::new();
    let bedrock_handler = BedrockPacketHandler::new(bedrock);

    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Socket Bind Error");

    let t_addr = address;
    let t_port = target_port;
    let t_ver = client_version.clone();

    tokio::spawn(async move {
        start_network_thread(
            socket,
            t_addr,
            t_port,
            t_ver,
            debug,
            raknet_handler,
            bedrock_handler,
            rx_outbound, // Listen to the commands from the game
            tx_inbound   // Send a packet to the game
        ).await;
    });

    Option::from(Client {
        network_sender: tx_outbound,
        network_receiver: rx_inbound,
        target_address,
        target_port,
        client_version,
        debug,
        auth_callback,
        chunk_palette_hashed: HashMap::new(),
        chunk_palette_runtime: vec![],
        chunk_air_id: 0,
        runtime_id: 0,
        unique_id: 0,
        current_tick: 0,
        player_position: vec![0.0, 0.0, 0.0],
        yaw: 0.0,
        pitch: 0.0,
    })
}

impl Client {

    /// When you call this function, the packet is passed to the background thread.
    pub fn send_packet(&self, packet_data: Box<dyn Packet>) {
        self.network_sender.send(packet_data).expect("Network thread closed, packet could not be sent.");
    }

    pub async fn next_event(&mut self) -> Option<(String, BedrockPacket)> {
        match self.network_receiver.recv().await {
            Some(event) => match event {
                ClientEvent::GameStarted { hashed_ids, runtime_ids, air_id, runtime_id, unique_id, current_tick, player_position, yaw, pitch } => {
                    if self.debug { println!("Block Palette Synchronized! ({} block)", if runtime_ids.len() != 0 { runtime_ids.len() } else { hashed_ids.len() }); }
                    self.chunk_palette_hashed = hashed_ids;
                    self.chunk_palette_runtime = runtime_ids;
                    self.chunk_air_id = air_id;
                    self.runtime_id = runtime_id;
                    self.unique_id = unique_id;
                    self.current_tick = current_tick;
                    self.player_position = player_position;
                    self.yaw = yaw;
                    self.pitch = pitch;
                    None
                },
                ClientEvent::Packet(name, pkt) => Some((name, pkt)),
            },
            None => None,
        }
    }

    pub fn get_sender(&self) -> UnboundedSender<Box<dyn Packet>> {
        self.network_sender.clone()
    }

    /// Auth callback setter function
    pub fn set_auth_callback<F>(&mut self, callback: F)
    where
        F: Fn(&str, &str) + Send + 'static,
    {
        *self.auth_callback.lock().unwrap() = Some(Box::new(callback));
    }

    pub fn print_chunk(&self, chunk_x: i32, chunk_z: i32, chunk: Chunk) {
        if self.chunk_palette_runtime.is_empty() && self.chunk_palette_hashed.is_empty() {
            println!("⚠️ The chunk packet has arrived, but there is no pallet data yet, and the block names cannot be resolved.");
            return;
        }
        for (sub_chunk_index, sub_chunk) in chunk.sub.iter().enumerate() {
            for (layer_index, storage) in sub_chunk.storages.iter().enumerate() {
                if layer_index == 0 {
                    for y in 0..16 {
                        for x in 0..16 {
                            for z in 0..16 {
                                let block_id = storage.at(x as u8, y as u8, z as u8);
                                let block_info = if !self.chunk_palette_hashed.is_empty() {
                                    self.chunk_palette_hashed.get(&block_id)
                                } else {
                                    self.chunk_palette_runtime.get(block_id as usize)
                                };

                                if let Some(tag) = block_info {
                                    if let Some(name) = tag.get_string("name") {
                                        if name != "minecraft:air" {
                                            let real_x = chunk_x * 16 + x;
                                            let real_y = chunk.r.0 + (sub_chunk_index * 16 + y) as isize;
                                            let real_z = chunk_z * 16 + z;
                                            println!("Block: {} Coordinate: {},{},{}", name, real_x, real_y, real_z);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// =================================================================================
// BACKGROUND NETWORK THREAD
// =================================================================================
async fn start_network_thread(
    socket: UdpSocket,
    target_address: String,
    target_port: u16,
    client_version: String,
    debug: bool,
    mut raknet_handler: RakNetPacketHandler,
    mut bedrock_handler: BedrockPacketHandler,
    mut rx_from_game: UnboundedReceiver<Box<dyn Packet>>,
    tx_to_game: UnboundedSender<ClientEvent>
) {
    if debug { println!("Connecting to {}:{}...", target_address, target_port); }
    socket.connect(format!("{}:{}", target_address, target_port)).await.expect("Socket connect fail");

    let mut req1 = Writer::new();
    OpenConnReq1::new(MAGIC, RAKNET_PROTOCOL_VERSION, 1492).encode(&mut req1);
    socket.send(req1.as_slice()).await.expect("Open Connection Request 1 packet could not be sent");

    let mut buffer = vec![0; 2048];
    let mut raknet_out = Writer::with_capacity(1500);   // handle_packet output
    let mut game_body = Writer::with_capacity(2048);   // packet body
    let mut datagram_out = Writer::with_capacity(1500); // datagram
    let mut ack_buf = Writer::with_capacity(64);       // ACK/NACK
    let mut game_scratch = vec![0u8; 16 * 1024 * 1024]; // decompress (16 MB)
    let mut should_stop = false;

    loop {
        if should_stop { break; }
        tokio::select! {
            // ------------------------------------------------------------------
            // A. OUTBOUND (Giden Paketler - Producer)
            // Oyundan gelen paketleri al, RakNet ile paketle ve gönder
            // ------------------------------------------------------------------
            Some(mut packet_data) = rx_from_game.recv() => {
                raknet_handler.game.encode(&mut *packet_data, &mut game_body).expect("Something went wrong");
                let datagrams = Datagram::split_packet(game_body.as_slice(), &mut raknet_handler.frame_number_cache);
                send_datagrams(&socket, &mut datagram_out, datagrams).await;
            }

            // ------------------------------------------------------------------
            // B. INBOUND (Gelen Paketler)
            // ------------------------------------------------------------------
            Ok((size, _addr)) = socket.recv_from(&mut buffer) => {
                let mut stream = Reader::new(&buffer[..size]);

                let packet_id = stream.get_u8();
                let packet_type = PacketType::from_byte(packet_id);

                let response = raknet_handler.handle_packet(&mut should_stop, debug, target_address.clone(), target_port, packet_type, &mut stream, &mut raknet_out);
                if !response.is_empty() {
                    socket.send(response).await.expect("RakNet Packet Error");
                }

                if !frame_set::is_datagram(packet_id) { continue; }

                let datagram = Datagram::from_binary(stream.get_buffer());

                ////////////////// SENDING ACK
                ack_buf.clear();
                Acknowledge::create(PacketType::ACK, 1, true, Option::from(datagram.sequence_number.clone()), None, None).encode(&mut ack_buf);
                socket.send(ack_buf.as_slice()).await.expect("ACK Send Error");
                //eprintln!("ACK seq={} bytes={:02X?}", datagram.sequence_number, ack_buf.as_slice());
                //////////////////

                let seq = datagram.sequence_number;

                for frame in datagram.frames {
                    if let Some(reliable_frame_index) = frame.reliable_frame_index {
                        // RELIABLE PACKET
                        raknet_handler.last_received_packets.insert(reliable_frame_index, frame);
                    } else {
                        // UNRELIABLE PACKET + HANDLER
                        let mut stream = Reader::new(frame.body.as_slice());
                        let packet_id = stream.get_u8();
                        let packet_type = PacketType::from_byte(packet_id);

                        let response_raknet_packet = raknet_handler.handle_packet(&mut should_stop, debug, target_address.clone(), target_port, packet_type, &mut stream, &mut raknet_out);
                        if !response_raknet_packet.is_empty() {
                            socket.send(&response_raknet_packet).await.expect("RakNet Packet Error");
                        }
                    }
                }

                // SENDING NACK
                /*if (raknet_handler.last_received_sequence_number + 1) != (seq as i64) {
                    for seq_num in ((raknet_handler.last_received_sequence_number+1) as u32)..seq {
                        let mut nack = Writer::new();
                        Acknowledge::create(PacketType::NACK, 1, true, Option::from(seq_num), None, None).encode(&mut nack);
                        socket.send(nack.as_slice()).await.expect("NACK Send Error");
                    }
                }*/
                if (seq as i64) > raknet_handler.last_received_sequence_number {
                    raknet_handler.last_received_sequence_number = seq as i64;
                }


                let mut sorted_reliable_frame_index: Vec<u32> = raknet_handler.last_received_packets.keys().cloned().collect();
                sorted_reliable_frame_index.sort();

                // fragment suspect
                for reliable_frame_index in sorted_reliable_frame_index {
                    if (reliable_frame_index as i64) <= raknet_handler.last_handled_reliable_frame_index { //////////////////////////////////////////////////////////////////////////////
                        raknet_handler.last_received_packets.remove(&reliable_frame_index);
                        continue;
                    }
                    if (reliable_frame_index as i64) == raknet_handler.last_handled_reliable_frame_index + 1 {
                        if let Some(frame) = raknet_handler.last_received_packets.get(&reliable_frame_index) {
                            let mut real_body = frame.body.to_vec();

                            // FRAGMENT HANDLER
                            if let Some(fragment) = &frame.fragment {
                                raknet_handler.last_received_fragment_packets.entry(fragment.compound_id).or_insert_with(HashMap::new).insert(fragment.index, frame.body.to_vec());
                                if let Some(fragment_data) = raknet_handler.last_received_fragment_packets.get(&fragment.compound_id) {
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
                                        raknet_handler.last_handled_reliable_frame_index = reliable_frame_index as i64;
                                        raknet_handler.last_received_packets.remove(&reliable_frame_index);
                                        continue;
                                    }
                                } else {
                                    raknet_handler.last_handled_reliable_frame_index = reliable_frame_index as i64;
                                    raknet_handler.last_received_packets.remove(&reliable_frame_index);
                                    continue;
                                }
                            }

                            // PACKET HANDLER
                            let mut stream = Reader::new(&real_body);
                            let packet_id = stream.get_u8();
                            let packet_type = PacketType::from_byte(packet_id);

                            match packet_type {
                                PacketType::NACK => {
                                    let nack = Acknowledge::decode(stream.get_buffer());
                                    if debug { nack.debug(true); }
                                }
                                PacketType::ConnectedPing => {
                                    let connected_ping = ConnectedPing::decode(stream.get_buffer());
                                    if debug { connected_ping.debug(); }

                                    let mut connected_pong = Writer::new();
                                    ConnectedPong::create(connected_ping.ping_time, Utc::now().timestamp() as u64).encode(&mut connected_pong);
                                    let frame = Datagram::create_frame(connected_pong.as_slice(), UNRELIABLE, &raknet_handler.frame_number_cache, None);
                                    let mut datagram = Writer::new();
                                    Datagram::create(vec![frame], &raknet_handler.frame_number_cache).to_binary(&mut datagram);
                                    raknet_handler.frame_number_cache.sequence_number += 1;
                                    socket.send(datagram.as_slice()).await.expect("ConnectedPong Packet could not be sent");
                                },
                                PacketType::ConnectedPong => {
                                    let connected_pong = ConnectedPong::decode(stream.get_buffer());
                                    if debug { connected_pong.debug(); }
                                    /*let connected_ping = connected_ping::create(Utc::now().timestamp()).encode();
                                    let frame = Datagram::create_frame(connected_ping, UNRELIABLE, &frame_number_cache, None);
                                    let datagram = Datagram::create(vec![frame], &frame_number_cache).to_binary();
                                    frame_number_cache.sequence_number += 1;
                                    socket.send(&datagram).await.expect("ConnectedPing Packet could not be sent");*/
                                },
                                PacketType::ConnReqAccepted => {
                                    let response = raknet_handler.handle_packet(&mut should_stop, debug, target_address.clone(), target_port, PacketType::ConnReqAccepted, &mut stream, &mut raknet_out);
                                    if !response.is_empty() {
                                        socket.send(response).await.expect("RakNet Packet Error");
                                    }
                                },
                                PacketType::Game => {
                                    //println!("Encryption {}, Compression {}", encryption_enabled, compression_enabled);
                                    let mut body = stream.remaining().to_vec();
                                    let mut stream = raknet_handler.game.decode(&mut body, &mut game_scratch).expect("GamePacket decode error");
                                    while !stream.feof() {
                                        let length = stream.get_var_u32();

                                        let packet_vec = stream.get(length as usize);
                                        let mut packet_stream = Reader::new(packet_vec);

                                        let packet_id = packet_stream.get_var_u32();
                                        //let packet_type = BedrockPacketType::from_byte(packet_id as u16);

                                        let packet = BedrockPacketType::get_packet_from_id(packet_id as u16, &mut packet_stream);

                                        match &packet {
                                            BedrockPacket::NetworkSettings(network_settings) => {
                                                raknet_handler.game = GamePacket::new(None, true, network_settings.compression_algorithm as u8);
                                                bedrock_handler.compression_enabled = true;

                                                // LOGIN PACKET
                                                let login_data_detail = login::convert_login_chain(&mut bedrock_handler.chain, &bedrock_handler.signing_key, bedrock_handler.signed_token.clone(), target_address.clone(), target_port, raknet_handler.client_guid, client_version.clone());
                                                let mut login = Login { client_protocol: BEDROCK_PROTOCOL_VERSION, auth_info_json: login_data_detail[0].clone(), client_data_jwt: login_data_detail[1].clone() };

                                                game_body.clear();
                                                raknet_handler.game.encode(&mut login, &mut game_body).expect("Something went wrong");
                                                let datagrams = Datagram::split_packet(game_body.as_slice(), &mut raknet_handler.frame_number_cache);
                                                send_datagrams(&socket, &mut datagram_out, datagrams).await;
                                            },
                                            BedrockPacket::ServerToClientHandshake(s2c_handshake) => {
                                                let jwt = String::from_utf8(s2c_handshake.jwt.clone()).unwrap();

                                                let jwt_split: Vec<&str> = jwt.split('.').collect();

                                                let jwt_header = Encryption::b64_url_decode(jwt_split[0]).unwrap();
                                                let jwt_header_value: Value = serde_json::from_str(jwt_header.as_str()).expect("JWT Header can not decoded.");

                                                let jwt_payload = Encryption::b64_url_decode(jwt_split[1]).unwrap();
                                                let jwt_payload_value: Value = serde_json::from_str(jwt_payload.as_str()).expect("JWT Payload can not decoded.");

                                                let x5u = jwt_header_value.get("x5u").and_then(Value::as_str).unwrap().to_string();
                                                let x5u_bytes = general_purpose::STANDARD.decode(x5u).expect("x5u decode error");
                                                let server_private = encryption::parse_der_public_key(x5u_bytes.as_slice());

                                                // decode_block removed
                                                //let salt = decode_block(jwt_payload_value.get("salt").and_then(Value::as_str).unwrap()).expect("Salt value cannot be decoded.");
                                                let padded = encryption::fix_base64_padding(jwt_payload_value.get("salt").and_then(Value::as_str).unwrap());
                                                let salt = general_purpose::STANDARD.decode(padded).expect("Salt value can not be decoded.");

                                                let shared_secret = encryption::generate_shared_secret(&bedrock_handler.signing_key, &server_private);
                                                let encryption_key = encryption::generate_key(&shared_secret, salt);
                                                let encryption = Encryption::fake_gcm(&encryption_key).expect("Encryption Fake GCM Error");

                                                raknet_handler.game = GamePacket::new(Option::from(encryption), bedrock_handler.compression_enabled, raknet_handler.game.compression_type);
                                                bedrock_handler.encryption_enabled = true;

                                                // CLIENT-TO-SERVER HANDSHAKE PACKET
                                                let mut c2s_handshake = ClientToServerHandshake{};

                                                raknet_handler.game.encode(&mut c2s_handshake, &mut game_body).expect("Something went wrong");
                                                let datagrams = Datagram::split_packet(game_body.as_slice(), &mut raknet_handler.frame_number_cache);
                                                send_datagrams(&socket, &mut datagram_out, datagrams).await;
                                            },
                                            BedrockPacket::ResourcePacksInfo(_resource_packs_info) => {
                                                /*
                                                let mut rp_uuids = Vec::new();
                                                for (_, resource_pack) in resource_packs_info.resource_packs.iter().enumerate() {
                                                    rp_uuids.push(resource_pack.uuid.clone());
                                                }*/

                                                // RESOURCE PACK CLIENT RESPONSE PACKET {HAVE ALL PACKS}
                                                let mut rp_client_response = ResourcePackClientResponse{ status: ResourcePackClientResponse::HAVE_ALL_PACKS, pack_ids: vec![] };

                                                raknet_handler.game.encode(&mut rp_client_response, &mut game_body).expect("Something went wrong");
                                                let datagrams = Datagram::split_packet(game_body.as_slice(), &mut raknet_handler.frame_number_cache);
                                                send_datagrams(&socket, &mut datagram_out, datagrams).await;

                                                // CLIENT CACHE STATUS PACKET
                                                let mut client_cache_status = ClientCacheStatus{ enabled: false };

                                                raknet_handler.game.encode(&mut client_cache_status, &mut game_body).expect("Something went wrong");
                                                let datagrams = Datagram::split_packet(game_body.as_slice(), &mut raknet_handler.frame_number_cache);
                                                send_datagrams(&socket, &mut datagram_out, datagrams).await;
                                            },
                                            BedrockPacket::ResourcePackStack(_resource_pack_stack) => {
                                                /*
                                                let mut pack_ids = vec![];
                                                for resource_stack_entry in &resource_pack_stack.resource_pack_stack {
                                                    pack_ids.push(resource_stack_entry.pack_id.clone());
                                                }*/

                                                // RESOURCE PACK CLIENT RESPONSE PACKET {COMPLETED}
                                                let mut rp_client_response = ResourcePackClientResponse{ status: ResourcePackClientResponse::COMPLETED, pack_ids: vec![] };

                                                raknet_handler.game.encode(&mut rp_client_response, &mut game_body).expect("Something went wrong");
                                                let datagrams = Datagram::split_packet(game_body.as_slice(), &mut raknet_handler.frame_number_cache);
                                                send_datagrams(&socket, &mut datagram_out, datagrams).await;
                                            },
                                            BedrockPacket::PlayStatus(play_status) => {
                                                if play_status.status == 3 { // Player Spawn
                                                    // SET LOCAL PLAYER AS INITIALIZED PACKET
                                                    let mut set_local_player_as_init = SetLocalPlayerAsInitializedPacket{ actor_runtime_id: 0 };

                                                    raknet_handler.game.encode(&mut set_local_player_as_init, &mut game_body).expect("Something went wrong");
                                                    let datagrams = Datagram::split_packet(game_body.as_slice(), &mut raknet_handler.frame_number_cache);
                                                    send_datagrams(&socket, &mut datagram_out, datagrams).await;
                                                }
                                            },
                                            BedrockPacket::StartGame(start_game) => {
                                                // --- HAFİF İŞ (network thread'de kalır) ---
                                                let mut req_chunk_radius = RequestChunkRadius { radius: 40, max_radius: 40 };
                                                raknet_handler.game.encode(&mut req_chunk_radius, &mut game_body).expect("encode");
                                                let datagrams = Datagram::split_packet(game_body.as_slice(), &mut raknet_handler.frame_number_cache);
                                                send_datagrams(&socket, &mut datagram_out, datagrams).await;

                                                // --- AĞIR İŞ (ayrı thread) ---
                                                let palette   = start_game.block_palette.clone();
                                                let hashes    = start_game.block_network_ids_are_hashes;
                                                let rid       = start_game.actor_runtime_id;
                                                let uid       = start_game.actor_unique_id;
                                                let tick      = start_game.current_tick;
                                                let pos  = start_game.player_position.clone();
                                                let yaw       = start_game.yaw;
                                                let pitch     = start_game.pitch;
                                                let tx        = tx_to_game.clone();

                                                tokio::task::spawn_blocking(move || {
                                                    let r = build_palette(&palette, hashes);
                                                    let _ = tx.send(ClientEvent::GameStarted {
                                                        hashed_ids:  r.hashed_ids,
                                                        runtime_ids: r.runtime_ids,
                                                        air_id:      r.air_id,
                                                        runtime_id:  rid,
                                                        unique_id:   uid,
                                                        current_tick: tick,
                                                        player_position: pos,
                                                        yaw,
                                                        pitch,
                                                    });
                                                });
                                            },
                                            BedrockPacket::AvailableCommands(_available_commands) => {
                                                // REQUEST CHUNK RADIUS PACKET
                                                let mut req_chunk_radius = RequestChunkRadius { radius: 40, max_radius: 40 };
                                                raknet_handler.game.encode(&mut req_chunk_radius, &mut game_body).expect("Something went wrong");
                                                let datagrams = Datagram::split_packet(game_body.as_slice(), &mut raknet_handler.frame_number_cache);
                                                send_datagrams(&socket, &mut datagram_out, datagrams).await;
                                            },
                                            BedrockPacket::LevelChunk(_level_chunk) => {
                                                /*if level_chunk.sub_chunk_count != 4294967294 {
                                                    let chunk = network_decode(bedrock_handler.air_network_id.clone(), level_chunk.extra_payload.clone(), level_chunk.sub_chunk_count, get_dimension_chunk_bounds(0));
                                                    if chunk.is_ok() {
                                                        print_all_blocks(level_chunk.chunk_x.clone(), level_chunk.chunk_z.clone(), chunk.unwrap());
                                                    } else {
                                                        panic!("{}", chunk.err().unwrap());
                                                    }
                                                }*/
                                            },
                                            BedrockPacket::NetworkStackLatency(network_stack_latency) => {
                                                if network_stack_latency.need_response { // send
                                                    // NETWORK STACK LATENCY
                                                    raknet_handler.game.encode(&mut NetworkStackLatency::response(network_stack_latency.timestamp), &mut game_body).expect("Something went wrong");
                                                    let datagrams = Datagram::split_packet(game_body.as_slice(), &mut raknet_handler.frame_number_cache);
                                                    send_datagrams(&socket, &mut datagram_out, datagrams).await;
                                                }
                                            },
                                            BedrockPacket::Disconnect(_) => { should_stop = true; },
                                            _ => {}
                                        }

                                        let packet_name = BedrockPacketType::get_packet_name(packet_id as u16).to_string();
                                        if let Err(_e) = tx_to_game.send(ClientEvent::Packet(packet_name, packet)) { continue };
                                    }
                                },
                                PacketType::DisconnectionNotification => {
                                    println!("{}Disconnect Notification Packet Received{}", COLOR_RED, COLOR_WHITE);
                                    should_stop = true;
                                }
                                _ => {}
                            }
                            raknet_handler.last_handled_reliable_frame_index = reliable_frame_index as i64;
                            raknet_handler.last_received_packets.remove(&reliable_frame_index);
                        }
                    }
                }

            }
        }
    }
}

async fn send_datagrams(socket: &UdpSocket, buf: &mut Writer, datagrams: Vec<Datagram>) {
    for d in datagrams {
        buf.clear();
        d.to_binary(buf);
        if !buf.is_empty() {
            let _ = socket.send(buf.as_slice()).await;
        }
    }
}

fn lookup_host(hostname: &String) -> io::Result<Vec<SocketAddr>> {
    (hostname.to_string(), 0).to_socket_addrs().map(|addrs| addrs.collect())
}

pub struct PaletteResult {
    pub hashed_ids: HashMap<u32, CompoundTag>,
    pub runtime_ids: Vec<CompoundTag>,
    pub air_id: u32,
}

fn build_palette(
    block_palette: &Vec<BlockPaletteEntry>,
    ids_are_hashes: bool,
) -> PaletteResult {
    let mut hashed_ids: HashMap<u32, CompoundTag> = HashMap::new();
    let mut runtime_ids: Vec<CompoundTag> = Vec::new();
    let mut air_id: u32 = 0;

    // --- Collect custom blocks ---
    let mut custom_blocks = HashMap::new();
    for entry in block_palette {
        let root = entry.get_states().get_root();
        if let Tag::Compound(bct) = root {
            let vanilla_block_data = bct.get_compound_tag("vanilla_block_data".to_string());
            let properties = bct.get_list_tag("properties".to_string());

            let mut properties_map = LinkedHashMap::new();

            if let Some(props) = properties {
                props.get_value().iter().for_each(|property| {
                    let mut property_enums_map: Vec<PropertyValue> = vec![];
                    if let Tag::Compound(pct) = property {
                        let property_name = pct.get_string("name").unwrap();
                        let property_enums = pct.get_list_tag("enum".to_string()).unwrap();
                        property_enums.get_value().iter().for_each(|property_enum| {
                            let id = property_enum.get_id();
                            if id == NBT::TAG_BYTE {
                                if let Tag::Byte(pce) = property_enum {
                                    property_enums_map.push(PropertyValue::Byte(pce.get_value()));
                                }
                            } else if id == NBT::TAG_STRING {
                                if let Tag::String(pce) = property_enum {
                                    property_enums_map.push(PropertyValue::Str(pce.get_value().clone()));
                                }
                            } else if id == NBT::TAG_INT {
                                if let Tag::Int(pce) = property_enum {
                                    property_enums_map.push(PropertyValue::Int(pce.get_value()));
                                }
                            } else {
                                println!("Unknown property enum id {:?}", id);
                            }
                        });
                        properties_map.insert(property_name, property_enums_map);
                    }
                });
            }

            let vbd = vanilla_block_data.unwrap();
            let block_id = vbd.get_int("block_id").unwrap();
            let block_data = format!("{}/{}", block_id, entry.get_name());
            custom_blocks.insert(block_data, properties_map);
        }
    }

    // --- Vanilla palette'i aç ---
    let cursor = Cursor::new(VANILLA_BLOCK_PALETTE);
    let mut decoder = GzDecoder::new(cursor);
    let mut contents = Vec::new();
    decoder.read_to_end(&mut contents).unwrap();
    let mut stream = Reader::new(contents.as_slice());

    let mut nbt_reader = NBTReader::new_big_endian();
    let mut offset = stream.offset();
    let nbt_root = nbt_reader.read(stream.get_buffer(), &mut offset, 0);
    stream.set_offset(offset);

    let ct = nbt_root.must_get_compound_tag().unwrap();
    let vanilla_blocks = ct.get_list_tag("blocks".to_string()).unwrap();

    if ids_are_hashes {
        for i in 0..vanilla_blocks.count() {
            if let Tag::Compound(mut vct) = vanilla_blocks.get(i) {
                let hashed_network_id = vct.get_int("network_id").unwrap() as u32;
                vct.remove_tag(vec![
                    "network_id".to_string(),
                    "name_hash".to_string(),
                    "version".to_string(),
                ]);
                hashed_ids.insert(hashed_network_id, vct);
            }
        }

        for (block_data, properties) in custom_blocks {
            let parts: Vec<&str> = block_data.split('/').collect();
            let block_id = parts[0].parse::<i32>().unwrap();
            let block_name = parts[1];

            for combo in block::cartesian_product_enum(&properties) {
                let mut state = CompoundTag::new(LinkedHashMap::new());
                for (k, v) in &combo {
                    match v {
                        PropertyValue::Int(i) => { state.set_int(k, *i); },
                        PropertyValue::Str(s) => { state.set_string(k, s.clone()); },
                        PropertyValue::Byte(b) => { state.set_byte(k, *b); },
                    }
                }

                let mut custom_ct = CompoundTag::new(LinkedHashMap::new());
                custom_ct.set_string("name", block_name.to_string());
                custom_ct.set_tag("states", Tag::Compound(state));

                let root = TreeRoot::new(Tag::Compound(custom_ct.clone()), "");
                let mut writer = NBTWriter::new_little_endian();
                let data = writer.write(root);

                let mut custom_ct_list = custom_ct;
                custom_ct_list.set_int("block_id", block_id);
                hashed_ids.insert(block::fnv1a_32(data), custom_ct_list);
            }
        }

        for (id, tag) in &hashed_ids {
            if tag.get_string("name").unwrap() == "minecraft:air" {
                air_id = *id;
                break;
            }
        }
    } else {
        let mut name_hashes: Vec<CompoundTag> = Vec::new();

        for i in 0..vanilla_blocks.count() {
            if let Tag::Compound(mut vct) = vanilla_blocks.get(i) {
                vct.remove_tag(vec!["version".to_string(), "network_id".to_string()]);
                name_hashes.push(vct);
            }
        }

        for (block_data, properties) in custom_blocks {
            let parts: Vec<&str> = block_data.split('/').collect();
            let block_id = parts[0].parse::<i32>().unwrap();
            let block_name = parts[1].to_string();

            for combo in block::cartesian_product_enum(&properties) {
                let mut state = CompoundTag::new(LinkedHashMap::new());
                for (k, v) in &combo {
                    match v {
                        PropertyValue::Int(i) => { state.set_int(k, *i); },
                        PropertyValue::Str(s) => { state.set_string(k, s.clone()); },
                        PropertyValue::Byte(b) => { state.set_byte(k, *b); },
                    }
                }

                let mut cct = CompoundTag::new(LinkedHashMap::new());
                cct.set_string("name", block_name.clone());
                cct.set_long("name_hash", block::fnv1_64(block_name.as_bytes()) as i64);
                cct.set_int("block_id", block_id);
                cct.set_tag("states", Tag::Compound(state));
                name_hashes.push(cct);
            }
        }

        name_hashes.sort_by_key(|tag| tag.get_long("name_hash").unwrap() as u64);

        if let Some(index) = name_hashes.iter().position(|t| t.get_string("name").unwrap() == "minecraft:air") {
            air_id = index as u32;
        }

        runtime_ids = name_hashes;
    }

    PaletteResult { hashed_ids, runtime_ids, air_id }
}