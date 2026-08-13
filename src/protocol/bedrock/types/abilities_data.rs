use crate::protocol::bedrock::types::abilities_layer::AbilitiesLayer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AbilitiesData {
    target_actor_unique_id: i64,
    player_permission: u8,
    command_permission: u8,
    ability_layers: Vec<AbilitiesLayer>,
}

impl AbilitiesData {
    pub fn new(
        target_actor_unique_id: i64,
        player_permission: u8,
        command_permission: u8,
        ability_layers: Vec<AbilitiesLayer>,
    ) -> AbilitiesData {
        AbilitiesData {
            target_actor_unique_id,
            player_permission,
            command_permission,
            ability_layers,
        }
    }

    pub fn read(stream: &mut Reader) -> AbilitiesData {
        let target_actor_unique_id = stream.get_i64_le();
        let player_permission = stream.get_u8();
        let command_permission = stream.get_u8();

        let ability_layers_count = stream.get_u8();
        let mut ability_layers = Vec::new();
        for _ in 0..ability_layers_count {
            ability_layers.push(AbilitiesLayer::read(stream));
        }

        AbilitiesData {
            target_actor_unique_id,
            player_permission,
            command_permission,
            ability_layers,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_i64_le(self.target_actor_unique_id);
        stream.put_u8(self.player_permission);
        stream.put_u8(self.command_permission);
        stream.put_u8(self.ability_layers.len() as u8);
        for ability_layer in &self.ability_layers {
            ability_layer.write(stream);
        }
    }
}
