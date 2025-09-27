use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct LevelSoundEvent {
    pub sound: u32,
    pub position: Vec<f32>,
    pub extra_data: i32,
    pub entity_type: String, //default ":"
    pub is_baby_mob: bool,
    pub disable_relative_volume: bool,
    pub actor_unique_id: i64,
}

pub fn new(sound: u32, position: Vec<f32>, extra_data: i32, entity_type: String, is_baby_mob: bool, disable_relative_volume: bool, actor_unique_id: i64,) -> LevelSoundEvent {
    LevelSoundEvent { sound, position, extra_data, entity_type, is_baby_mob, disable_relative_volume, actor_unique_id }
}

impl Packet for LevelSoundEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLevelSoundEvent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.sound);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        stream.put_var_int(self.extra_data);
        PacketSerializer::put_string(&mut stream, self.entity_type.clone());
        stream.put_bool(self.is_baby_mob);
        stream.put_bool(self.disable_relative_volume);
        stream.put_l_long(self.actor_unique_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> LevelSoundEvent {
        let mut stream = Stream::new(bytes, 0);

        let sound = stream.get_unsigned_var_int();
        let position = PacketSerializer::get_vector3(&mut stream);
        let extra_data = stream.get_var_int();
        let entity_type = PacketSerializer::get_string(&mut stream);
        let is_baby_mob = stream.get_bool();
        let disable_relative_volume = stream.get_bool();
        let actor_unique_id = stream.get_l_long();

        LevelSoundEvent { sound, position, extra_data, entity_type, is_baby_mob, disable_relative_volume, actor_unique_id }
    }

    fn debug(&self) {
        println!("Sound: {}", self.sound);
        println!("Position: {:?}", self.position);
        println!("Extra Data: {}", self.extra_data);
        println!("Entity Type: {}", self.entity_type);
        println!("Is Baby Mob: {}", self.is_baby_mob);
        println!("Disable Relative Volume: {}", self.disable_relative_volume);
        println!("Actor Unique ID: {}", self.actor_unique_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
