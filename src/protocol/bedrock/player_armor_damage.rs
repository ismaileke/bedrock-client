use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct PlayerArmorDamage {
    pub head_slot_damage: Option<i32>,
    pub chest_slot_damage: Option<i32>,
    pub legs_slot_damage: Option<i32>,
    pub feet_slot_damage: Option<i32>,
    pub body_slot_damage: Option<i32>
}

pub fn new(
    head_slot_damage: Option<i32>,
    chest_slot_damage: Option<i32>,
    legs_slot_damage: Option<i32>,
    feet_slot_damage: Option<i32>,
    body_slot_damage: Option<i32>
) -> PlayerArmorDamage {
    PlayerArmorDamage { head_slot_damage, chest_slot_damage, legs_slot_damage, feet_slot_damage, body_slot_damage }
}

impl PlayerArmorDamage {
    const FLAG_HEAD: u8 = 0;
    const FLAG_CHEST: u8 = 1;
    const FLAG_LEGS: u8 = 2;
    const FLAG_FEET: u8 = 3;
    const FLAG_BODY: u8 = 4;

    fn maybe_read_damage(flags: u8, flag: u8, stream: &mut Stream) -> Option<i32> {
        if flags & (1 << flag) != 0 {
            return Option::from(stream.get_var_int());
        }
        None
    }

    fn maybe_write_damage(damage: Option<i32>, stream: &mut Stream) {
        if let Some(damage) = damage {
            stream.put_var_int(damage);
        }
    }

    fn compose_flag(damage: Option<i32>, flag: u8) -> u8 {
        if let Some(_) = damage {
            return 1u8 << flag;
        }
        0
    }
}

impl Packet for PlayerArmorDamage {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerArmorDamage.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_byte(
            Self::compose_flag(self.head_slot_damage, Self::FLAG_HEAD) |
                Self::compose_flag(self.chest_slot_damage, Self::FLAG_CHEST) |
                Self::compose_flag(self.legs_slot_damage, Self::FLAG_LEGS) |
                Self::compose_flag(self.feet_slot_damage, Self::FLAG_FEET) |
                Self::compose_flag(self.body_slot_damage, Self::FLAG_BODY)
        );
        Self::maybe_write_damage(self.head_slot_damage, &mut stream);
        Self::maybe_write_damage(self.chest_slot_damage, &mut stream);
        Self::maybe_write_damage(self.legs_slot_damage, &mut stream);
        Self::maybe_write_damage(self.feet_slot_damage, &mut stream);
        Self::maybe_write_damage(self.body_slot_damage, &mut stream);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PlayerArmorDamage {
        let mut stream = Stream::new(bytes, 0);

        let flags = stream.get_byte();
        let head_slot_damage = Self::maybe_read_damage(flags, Self::FLAG_HEAD, &mut stream);
        let chest_slot_damage = Self::maybe_read_damage(flags, Self::FLAG_CHEST, &mut stream);
        let legs_slot_damage = Self::maybe_read_damage(flags, Self::FLAG_LEGS, &mut stream);
        let feet_slot_damage = Self::maybe_read_damage(flags, Self::FLAG_FEET, &mut stream);
        let body_slot_damage = Self::maybe_read_damage(flags, Self::FLAG_BODY, &mut stream);

        PlayerArmorDamage { head_slot_damage, chest_slot_damage, legs_slot_damage, feet_slot_damage, body_slot_damage }
    }

    fn debug(&self) {
        println!("Head Slot Damage: {:?}", self.head_slot_damage);
        println!("Chest Slot Damage: {:?}", self.chest_slot_damage);
        println!("Legs Slot Damage: {:?}", self.legs_slot_damage);
        println!("Feet Slot Damage: {:?}", self.feet_slot_damage);
        println!("Body Slot Damage: {:?}", self.body_slot_damage);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
