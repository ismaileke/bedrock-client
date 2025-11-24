#[derive(serde::Serialize, Clone, Debug)]
pub struct EntityLink{
    pub from_actor_unique_id: i64,
    pub to_actor_unique_id: i64,
    pub action_type: u8,
    pub immediate: bool,
    pub caused_by_rider: bool,
    pub vehicle_angular_velocity: f32,
}

impl EntityLink{
    pub const TYPE_REMOVE: u8 = 0;
    pub const TYPE_RIDER: u8 = 1;
    pub const TYPE_PASSENGER: u8 = 2;
    
    pub fn new(from_actor_unique_id: i64, to_actor_unique_id: i64, action_type: u8, immediate: bool, caused_by_rider: bool, vehicle_angular_velocity: f32) -> EntityLink{
        EntityLink{from_actor_unique_id, to_actor_unique_id, action_type, immediate, caused_by_rider, vehicle_angular_velocity}
    }
}