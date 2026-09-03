use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::inventory_transaction_changed_slots_hack::InventoryTransactionChangedSlotsHack;
use crate::protocol::bedrock::types::inventory::mismatch_transaction_data::MismatchTransactionData;
use crate::protocol::bedrock::types::inventory::normal_transaction_data::NormalTransactionData;
use crate::protocol::bedrock::types::inventory::release_item_transaction_data::ReleaseItemTransactionData;
use crate::protocol::bedrock::types::inventory::transaction_data::TransactionData;
use crate::protocol::bedrock::types::inventory::use_item_on_entity_transaction_data::UseItemOnEntityTransactionData;
use crate::protocol::bedrock::types::inventory::use_item_transaction_data::UseItemTransactionData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct InventoryTransaction {
    pub request_id: i32,
    pub request_changed_slots: Option<Vec<InventoryTransactionChangedSlotsHack>>,
    pub tr_data: Option<TransactionData>,
}

impl Packet for InventoryTransaction {
    fn id(&self) -> u16 {
        BedrockPacketType::IDInventoryTransaction.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::write_legacy_item_stack_request_id(stream, self.request_id);
        PacketSerializer::write_optional(stream, &self.request_changed_slots, |s, v| {
            s.put_var_u32(v.len() as u32);
            for changed_slot in v {
                changed_slot.write(s);
            }
        });
        PacketSerializer::write_optional(stream, &self.tr_data, |s, v| s.put_var_u32(v.get_type_id()));
        if let Some(tr_data) = &self.tr_data {
            tr_data.encode(stream);
        }
    }
    
    fn decode(stream: &mut Reader) -> InventoryTransaction {
        let request_id = PacketSerializer::read_legacy_item_stack_request_id(stream);
        let request_changed_slots = PacketSerializer::read_optional(stream, |s| {
            let slot_count = s.get_var_u32() as usize;
            let mut result = Vec::with_capacity(slot_count);
            for _ in 0..slot_count {
                result.push(InventoryTransactionChangedSlotsHack::read(s));
            }
            return result;
        });
        let tr_type = PacketSerializer::read_optional(stream, |s| s.get_var_u32());
        let mut tr_data = match tr_type {
            Some(Self::TYPE_NORMAL) => Some(TransactionData::Normal(NormalTransactionData::new(vec![]))),
            Some(Self::TYPE_MISMATCH) => Some(TransactionData::Mismatch(MismatchTransactionData::new())),
            Some(Self::TYPE_USE_ITEM) => Some(TransactionData::UseItem(UseItemTransactionData::null())),
            Some(Self::TYPE_USE_ITEM_ON_ENTITY) => Some(TransactionData::UseItemOnEntity(UseItemOnEntityTransactionData::null())),
            Some(Self::TYPE_RELEASE_ITEM) => Some(TransactionData::ReleaseItem(ReleaseItemTransactionData::null())),
            None => None,
            _ => panic!("Unknown transaction type: {:?}", tr_type),
        };
        if let Some(tr_data) = tr_data.as_mut() {
            tr_data.decode(stream);
        }

        InventoryTransaction { request_id, request_changed_slots, tr_data }
    }
}

impl InventoryTransaction {
    pub const TYPE_NORMAL: u32 = 0;
    pub const TYPE_MISMATCH: u32 = 1;
    pub const TYPE_USE_ITEM: u32 = 2;
    pub const TYPE_USE_ITEM_ON_ENTITY: u32 = 3;
    pub const TYPE_RELEASE_ITEM: u32 = 4;
}
