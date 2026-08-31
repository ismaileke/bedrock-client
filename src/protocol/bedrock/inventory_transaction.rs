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
    pub tr_data: TransactionData,
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
        stream.put_u8(1);
        stream.put_var_u32(self.tr_data.get_type_id());
        stream.put_u8(1);
        self.tr_data.encode(stream);
    }
    
    fn decode(stream: &mut Reader) -> InventoryTransaction {
        let request_id = PacketSerializer::read_legacy_item_stack_request_id(stream);
        let request_changed_slots = PacketSerializer::read_optional(stream, |s| {
            let mut result = Vec::new();
            let slot_count = s.get_var_u32() as usize;
            for _ in 0..slot_count {
                result.push(InventoryTransactionChangedSlotsHack::read(s));
            }
            return result;
        });
        if stream.get_u8() != 1 {
            panic!("Dummy optional bool for transactionType should always be 1");
        }
        let tr_type = stream.get_var_u32();
        if stream.get_u8() != 1 {
            panic!("Dummy optional bool for trData should always be 1");
        }
        let mut tr_data = match tr_type {
            Self::TYPE_NORMAL => TransactionData::Normal(NormalTransactionData::new(vec![])),
            Self::TYPE_MISMATCH => TransactionData::Mismatch(MismatchTransactionData::new()),
            Self::TYPE_USE_ITEM => TransactionData::UseItem(UseItemTransactionData::null()),
            Self::TYPE_USE_ITEM_ON_ENTITY => {
                TransactionData::UseItemOnEntity(UseItemOnEntityTransactionData::null())
            }
            Self::TYPE_RELEASE_ITEM => {
                TransactionData::ReleaseItem(ReleaseItemTransactionData::null())
            }
            _ => TransactionData::Normal(NormalTransactionData::new(vec![])),
        };
        tr_data.decode(stream);

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
