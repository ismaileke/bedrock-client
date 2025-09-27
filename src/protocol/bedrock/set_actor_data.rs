use std::any::Any;
use std::collections::HashMap;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;
use crate::protocol::bedrock::types::entity::property_sync_data::PropertySyncData;

pub struct SetActorData {
    pub actor_runtime_id: u64,
    pub metadata: HashMap<u32, Box<dyn MetadataProperty>>,
    pub synced_properties: PropertySyncData,
    pub tick: u64
}

pub fn new(actor_runtime_id: u64, metadata: HashMap<u32, Box<dyn MetadataProperty>>, synced_properties: PropertySyncData, tick: u64) -> SetActorData {
    SetActorData { actor_runtime_id, metadata, synced_properties, tick }
}

impl Packet for SetActorData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetActorData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        PacketSerializer::put_entity_metadata(&mut stream, &mut self.metadata);
        self.synced_properties.write(&mut stream);
        stream.put_unsigned_var_long(self.tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetActorData {
        let mut stream = Stream::new(bytes, 0);

        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let metadata = PacketSerializer::get_entity_metadata(&mut stream);
        let synced_properties = PropertySyncData::read(&mut stream);
        let tick = stream.get_unsigned_var_long();

        SetActorData { actor_runtime_id, metadata, synced_properties, tick }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Metadata: {:?}", self.metadata);
        println!("Synced Properties: {:?}", self.synced_properties);
        println!("Tick: {}", self.tick);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
