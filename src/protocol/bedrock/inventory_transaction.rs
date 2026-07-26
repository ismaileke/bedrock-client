use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::inventory_transaction_changed_slots_hack::InventoryTransactionChangedSlotsHack;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use crate::protocol::bedrock::types::inventory::mismatch_transaction_data::MismatchTransactionData;
use crate::protocol::bedrock::types::inventory::normal_transaction_data::NormalTransactionData;
use crate::protocol::bedrock::types::inventory::release_item_transaction_data::ReleaseItemTransactionData;
use crate::protocol::bedrock::types::inventory::transaction_data::TransactionData;
use crate::protocol::bedrock::types::inventory::use_item_on_entity_transaction_data::UseItemOnEntityTransactionData;
use crate::protocol::bedrock::types::inventory::use_item_transaction_data::UseItemTransactionData;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct InventoryTransaction {
    pub request_id: i32,
    pub request_changed_slots: Option<Vec<InventoryTransactionChangedSlotsHack>>,
    pub tr_data: TransactionData,
}

impl Packet for InventoryTransaction {
    fn id(&self) -> u16 {
        BedrockPacketType::IDInventoryTransaction.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::write_legacy_item_stack_request_id(&mut stream, self.request_id);
        PacketSerializer::write_optional(&mut stream, &self.request_changed_slots, |s, v| {
            s.put_var_u32(v.len() as u32);
            for changed_slot in v {
                changed_slot.write(s);
            }
        });
        stream.put_byte(1);
        stream.put_var_u32(self.tr_data.get_type_id());
        stream.put_byte(1);
        self.tr_data.encode_transaction(&mut stream);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> InventoryTransaction {
        let request_id = PacketSerializer::read_legacy_item_stack_request_id(stream);
        let request_changed_slots = PacketSerializer::read_optional(stream, |s| {
            let mut result = Vec::new();
            let slot_count = s.get_var_u32() as usize;
            for _ in 0..slot_count {
                result.push(InventoryTransactionChangedSlotsHack::read(s));
            }
            return result;
        });
        if stream.get_byte() != 1 {
            panic!("Dummy optional bool for transactionType should always be 1");
        }
        let tr_type = stream.get_var_u32();
        if stream.get_byte() != 1 {
            panic!("Dummy optional bool for trData should always be 1");
        }
        // check later, bad using
        let mut tr_data = match tr_type {
            Self::TYPE_NORMAL => TransactionData::Normal(NormalTransactionData::new(vec![])),
            Self::TYPE_MISMATCH => TransactionData::Mismatch(MismatchTransactionData::new()),
            Self::TYPE_USE_ITEM => TransactionData::UseItem(UseItemTransactionData::new(
                vec![],
                0,
                0,
                vec![],
                0,
                0,
                ItemStackWrapper {
                    stack_id: 0,
                    item_stack: ItemStack::null(),
                    variant: 0,
                },
                vec![],
                vec![],
                0,
                0,
                0
            )),
            Self::TYPE_USE_ITEM_ON_ENTITY => {
                TransactionData::UseItemOnEntity(UseItemOnEntityTransactionData::new(
                    vec![],
                    0,
                    0,
                    0,
                    ItemStackWrapper {
                        stack_id: 0,
                        item_stack: ItemStack::null(),
                        variant: 0,
                    },
                    vec![],
                    vec![],
                ))
            }
            Self::TYPE_RELEASE_ITEM => {
                TransactionData::ReleaseItem(ReleaseItemTransactionData::new(
                    vec![],
                    0,
                    0,
                    ItemStackWrapper {
                        stack_id: 0,
                        item_stack: ItemStack::null(),
                        variant: 0,
                    },
                    vec![],
                ))
            }
            _ => TransactionData::Normal(NormalTransactionData::new(vec![])),
        };
        tr_data.decode_transaction(stream);

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
