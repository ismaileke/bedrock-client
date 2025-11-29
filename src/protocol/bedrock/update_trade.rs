use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use mojang_nbt::tag::tag::Tag;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

#[derive(serde::Serialize, Debug)]
pub struct UpdateTrade {
    pub window_id: u8,
    pub window_type: u8,
    pub window_slot_count: i32,
    pub trade_tier: i32,
    pub trader_actor_unique_id: i64,
    pub player_actor_unique_id: i64,
    pub display_name: String,
    pub is_v2_trading: bool,
    pub is_economy_trading: bool,
    pub offers: CacheableNBT
}

pub fn new(
    window_id: u8,
    window_type: u8,
    window_slot_count: i32,
    trade_tier: i32,
    trader_actor_unique_id: i64,
    player_actor_unique_id: i64,
    display_name: String,
    is_v2_trading: bool,
    is_economy_trading: bool,
    offers: CacheableNBT
) -> UpdateTrade {
    UpdateTrade { window_id, window_type, window_slot_count, trade_tier, trader_actor_unique_id, player_actor_unique_id, display_name, is_v2_trading, is_economy_trading, offers }
}

impl Packet for UpdateTrade {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateTrade.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.window_id);
        stream.put_byte(self.window_type);
        stream.put_var_i32(self.window_slot_count);
        stream.put_var_i32(self.trade_tier);
        PacketSerializer::put_actor_unique_id(&mut stream, self.trader_actor_unique_id);
        PacketSerializer::put_actor_unique_id(&mut stream, self.player_actor_unique_id);
        PacketSerializer::put_string(&mut stream, self.display_name.clone());
        stream.put_bool(self.is_v2_trading);
        stream.put_bool(self.is_economy_trading);
        stream.put(self.offers.get_encoded_nbt());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> UpdateTrade {
        let window_id = stream.get_byte();
        let window_type = stream.get_byte();
        let window_slot_count = stream.get_var_i32();
        let trade_tier = stream.get_var_i32();
        let trader_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let player_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let display_name = PacketSerializer::get_string(stream);
        let is_v2_trading = stream.get_bool();
        let is_economy_trading = stream.get_bool();
        let offers = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(stream)));

        UpdateTrade { window_id, window_type, window_slot_count, trade_tier, trader_actor_unique_id, player_actor_unique_id, display_name, is_v2_trading, is_economy_trading, offers }
    }

    fn debug(&self) {
        println!("Window ID: {}", self.window_id);
        println!("Window Type: {}", self.window_type);
        println!("Window Slot Count: {}", self.window_slot_count);
        println!("Trade Tier: {}", self.trade_tier);
        println!("Trade Actor Unique ID: {}", self.trader_actor_unique_id);
        println!("Player Actor Unique ID: {}", self.player_actor_unique_id);
        println!("Display Name: {}", self.display_name);
        println!("Is V2 Trading: {}", self.is_v2_trading);
        println!("Is Economy Trading: {}", self.is_economy_trading);
        println!("Offers: {:?}", self.offers);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
