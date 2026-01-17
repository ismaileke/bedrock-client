use binary_utils::binary::Stream;
use std::collections::HashMap;
use log::error;

#[derive(serde::Serialize, Debug)]
pub struct AbilitiesLayer {
    layer_id: u16,
    bool_abilities: HashMap<u32, bool>,
    fly_speed: Option<f32>,
    vertical_fly_speed: Option<f32>,
    walk_speed: Option<f32>,
}

impl AbilitiesLayer {
    pub const LAYER_CACHE: u16 = 0;
    pub const LAYER_BASE: u16 = 1;
    pub const LAYER_SPECTATOR: u16 = 2;
    pub const LAYER_COMMANDS: u16 = 3;
    pub const LAYER_EDITOR: u16 = 4;
    pub const LAYER_LOADING_SCREEN: u16 = 5;

    pub const ABILITY_BUILD: u32 = 0;
    pub const ABILITY_MINE: u32 = 1;
    pub const ABILITY_DOORS_AND_SWITCHES: u32 = 2; //disabling this also disables dropping items (???)
    pub const ABILITY_OPEN_CONTAINERS: u32 = 3;
    pub const ABILITY_ATTACK_PLAYERS: u32 = 4;
    pub const ABILITY_ATTACK_MOBS: u32 = 5;
    pub const ABILITY_OPERATOR: u32 = 6;
    pub const ABILITY_TELEPORT: u32 = 7;
    pub const ABILITY_INVULNERABLE: u32 = 8;
    pub const ABILITY_FLYING: u32 = 9;
    pub const ABILITY_ALLOW_FLIGHT: u32 = 10;
    pub const ABILITY_INFINITE_RESOURCES: u32 = 11; //in vanilla, they call this "instabuild", which is a bad name
    pub const ABILITY_LIGHTNING: u32 = 12; //???
    const ABILITY_FLY_SPEED: u32 = 13;
    const ABILITY_WALK_SPEED: u32 = 14;
    pub const ABILITY_MUTED: u32 = 15;
    pub const ABILITY_WORLD_BUILDER: u32 = 16;
    pub const ABILITY_NO_CLIP: u32 = 17;
    pub const ABILITY_PRIVILEGED_BUILDER: u32 = 18;
    pub const ABILITY_VERTICAL_FLY_SPEED: u32 = 19;

    pub const NUMBER_OF_ABILITIES: u32 = 20;

    pub fn new(
        layer_id: u16,
        bool_abilities: HashMap<u32, bool>,
        fly_speed: Option<f32>,
        vertical_fly_speed: Option<f32>,
        walk_speed: Option<f32>,
    ) -> AbilitiesLayer {
        AbilitiesLayer {
            layer_id,
            bool_abilities,
            fly_speed,
            vertical_fly_speed,
            walk_speed,
        }
    }

    pub fn read(stream: &mut Stream) -> AbilitiesLayer {
        let layer_id = stream.get_u16_le();
        let set_abilities = stream.get_u32_le();
        let set_ability_values = stream.get_u32_le();

        let mut fly_speed = Option::from(stream.get_f32_le());
        let mut vertical_fly_speed = Option::from(stream.get_f32_le());
        let mut walk_speed = Option::from(stream.get_f32_le());

        let mut bool_abilities = HashMap::new();
        for i in 0..AbilitiesLayer::NUMBER_OF_ABILITIES {
            if i == Self::ABILITY_FLY_SPEED || i == Self::ABILITY_WALK_SPEED {
                continue;
            }
            if set_abilities & (1 << i) != 0 {
                bool_abilities.insert(i, set_ability_values & (1 << i) != 0);
            }
        }
        if set_abilities & (1 << Self::ABILITY_FLY_SPEED) == 0 {
            if fly_speed.unwrap() != 0.0 {
                error!("Fly speed should be zero if the layer does not set it");
            }
            fly_speed = None;
        }
        if set_abilities & (1 << Self::ABILITY_VERTICAL_FLY_SPEED) == 0 {
            if vertical_fly_speed.unwrap() != 0.0 {
                error!("Vertical Fly speed should be zero if the layer does not set it");
            }
            vertical_fly_speed = None;
        }
        if set_abilities & (1 << Self::ABILITY_WALK_SPEED) == 0 {
            if walk_speed.unwrap() != 0.0 {
                error!("Walk speed should be zero if the layer does not set it");
            }
            walk_speed = None;
        }

        AbilitiesLayer {
            layer_id,
            bool_abilities,
            fly_speed,
            vertical_fly_speed,
            walk_speed,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_u16_le(self.layer_id);

        let mut set_abilities = 0;
        let mut set_ability_values = 0;
        for (ability, value) in &self.bool_abilities {
            set_abilities |= 1 << *ability;
            set_ability_values |= if *value { 1 << *ability } else { 0 };
        }
        if self.fly_speed.is_some() {
            set_abilities |= 1 << Self::ABILITY_FLY_SPEED;
        }
        if self.vertical_fly_speed.is_some() {
            set_abilities |= 1 << Self::ABILITY_VERTICAL_FLY_SPEED;
        }
        if self.walk_speed.is_some() {
            set_abilities |= 1 << Self::ABILITY_WALK_SPEED;
        }

        stream.put_u32_le(set_abilities);
        stream.put_u32_le(set_ability_values);
        stream.put_f32_le(if self.fly_speed.is_some() {
            self.fly_speed.unwrap()
        } else {
            0.0
        });
        stream.put_f32_le(if self.vertical_fly_speed.is_some() {
            self.vertical_fly_speed.unwrap()
        } else {
            0.0
        });
        stream.put_f32_le(if self.walk_speed.is_some() {
            self.walk_speed.unwrap()
        } else {
            0.0
        });
    }
}
