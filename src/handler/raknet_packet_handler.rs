use crate::protocol::bedrock::network_settings::NONE;
use crate::protocol::bedrock::req_network_settings::RequestNetworkSettings;
use crate::protocol::raknet::conn_req::ConnReq;
use crate::protocol::raknet::conn_req_accepted::ConnReqAccepted;
use crate::protocol::raknet::connected_ping::ConnectedPing;
use crate::protocol::raknet::frame_set::{Datagram, Frame, FrameNumberCache, RELIABLE, RELIABLE_ORDERED, UNRELIABLE};
use crate::protocol::raknet::game_packet::GamePacket;
use crate::protocol::raknet::incompatible_protocol::IncompatibleProtocol;
use crate::protocol::raknet::new_incoming_conn::NewIncomingConn;
use crate::protocol::raknet::open_conn_reply1::OpenConnReply1;
use crate::protocol::raknet::open_conn_reply2::OpenConnReply2;
use crate::protocol::raknet::open_conn_req2::OpenConnReq2;
use crate::protocol::raknet::packet_ids::{PacketType, MAGIC};
use crate::protocol::raknet::frame_set;
use crate::utils::address::InternetAddress;
use crate::utils::color_format::{COLOR_RED, COLOR_WHITE};
use crate::BEDROCK_PROTOCOL_VERSION;
use binary_utils::binary::{Reader, Writer};
use chrono::Utc;
use rand::{rng, RngExt};
use std::collections::HashMap;

pub struct RakNetPacketHandler {
    pub client_guid: i64,
    pub game: GamePacket,
    pub frame_number_cache: FrameNumberCache,
    pub last_received_packets: HashMap<u32, Frame>, // reliable_frame_index: Frame
    pub last_received_fragment_packets: HashMap<u16, HashMap<u32, Vec<u8>>>, // split_id: index => buffer
    pub last_received_sequence_number: i64, // i64, u32'yi kapsadığı için kullandım (-1)
    pub last_handled_reliable_frame_index: i64,
    pub missing_datagrams: HashMap<u32, std::time::Instant>,
}

impl RakNetPacketHandler {
    pub fn new() -> RakNetPacketHandler {
        let mut rng = rng();
        let client_guid = rng.random_range(10000..100000);
        let game = GamePacket::new(None, false, NONE);
        let frame_number_cache = frame_set::start_number_cache();
        let last_received_packets = HashMap::new();
        let last_received_fragment_packets = HashMap::new();
        let last_received_sequence_number = -1;
        let last_handled_reliable_frame_index = -1;
        let missing_datagrams = HashMap::new();

        RakNetPacketHandler {
            client_guid,
            game,
            frame_number_cache,
            last_received_packets,
            last_received_fragment_packets,
            last_received_sequence_number,
            last_handled_reliable_frame_index,
            missing_datagrams,
        }
    }

    pub fn handle_packet<'b>(
        &mut self,
        should_stop: &mut bool,
        debug: bool,
        target_address: String,
        target_port: u16,
        packet_type: PacketType,
        stream: &mut Reader<'_>,
        out: &'b mut Writer
    ) -> &'b [u8] {
        out.clear();

        match packet_type {
            PacketType::OpenConnReply1 => {
                let open_conn_reply1 = OpenConnReply1::decode(stream.get_buffer());
                if debug { open_conn_reply1.debug(); }

                OpenConnReq2::new(MAGIC, InternetAddress::new(4, target_address.to_string(), target_port), open_conn_reply1.cookie, false, open_conn_reply1.mtu, self.client_guid as u64).encode(out);

                //client.socket.send(&req2).expect("Open Connection Request 2 Packet could not be sent");
            },
            PacketType::OpenConnReply2 => {
                let open_conn_reply2 = OpenConnReply2::decode(stream.get_buffer());
                if debug { open_conn_reply2.debug(); }

                let mut conn_req = Writer::with_capacity(64);
                ConnReq::new(self.client_guid as u64, Utc::now().timestamp() as u64, false).encode(&mut conn_req);

                let frame = Datagram::create_frame(conn_req.as_slice(), RELIABLE, &self.frame_number_cache, None);
                Datagram::create(vec![frame], &self.frame_number_cache).to_binary(out);
                self.frame_number_cache.sequence_number += 1;
                self.frame_number_cache.reliable_frame_index += 1;
                //client.socket.send(&datagram).expect("Connection Request Packet could not be sent");
            },
            PacketType::ConnReqAccepted => {
                let conn_req_accepted = ConnReqAccepted::decode(stream.get_buffer());
                if debug { conn_req_accepted.debug(); }

                let mut w1 = Writer::with_capacity(1500);
                let mut w2 = Writer::with_capacity(64);
                let mut w3 = Writer::with_capacity(64);

                // New Incoming Connection
                let addresses: [InternetAddress; 20] = core::array::from_fn(|_| InternetAddress::new(4, "0.0.0.0".to_string(), 0));
                NewIncomingConn::new(InternetAddress::new(4, target_address.to_string(), target_port), addresses, Utc::now().timestamp() as u64, (Utc::now().timestamp() + 1) as u64).encode(&mut w1);
                let frame = Datagram::create_frame(w1.as_slice(), RELIABLE_ORDERED, &self.frame_number_cache, None);
                self.frame_number_cache.reliable_frame_index += 1;
                self.frame_number_cache.ordered_frame_index += 1;

                // Connected Ping
                ConnectedPing::create(Utc::now().timestamp() as u64).encode(&mut w2);
                let frame_two = Datagram::create_frame(w2.as_slice(), UNRELIABLE, &self.frame_number_cache, None);

                // Request Network Settings Packet
                let mut req_network_settings = RequestNetworkSettings{ protocol_version: BEDROCK_PROTOCOL_VERSION };
                self.game.encode(&mut req_network_settings, &mut w3).expect("Something went wrong");
                let frame_three = Datagram::create_frame(w3.as_slice(), RELIABLE_ORDERED, &self.frame_number_cache, None);

                Datagram::create(vec![frame, frame_two, frame_three], &self.frame_number_cache).to_binary(out);
                self.frame_number_cache.sequence_number += 1;
                self.frame_number_cache.reliable_frame_index += 1;
                self.frame_number_cache.ordered_frame_index += 1;
                //client.socket.send(&datagram).expect("NewIncomingConnection & RequestNetworkSettings Packet could not be sent");
                //should_stop = true;
            },
            PacketType::IncompatibleProtocol => {
                let incompatible_protocol = IncompatibleProtocol::decode(stream.get_buffer());
                println!("{}Incompatible Protocol Version, Server Protocol Version: {}{}", COLOR_RED, incompatible_protocol.server_protocol, COLOR_WHITE);
                *should_stop = true;
            },
            PacketType::DisconnectionNotification => {
                println!("{}Disconnection Notification Packet Received From Server.{}", COLOR_RED, COLOR_WHITE);
                *should_stop = true;
            }
            _ => {}
        }
        out.as_slice()
    }
}
