use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct PlayerAuthInputVehicleInfo {
    vehicle_rotation_x: f32,
    vehicle_rotation_z: f32,
    predicted_vehicle_actor_unique_id: i64
}

impl PlayerAuthInputVehicleInfo {
    pub fn new(vehicle_rotation_x: f32, vehicle_rotation_z: f32, predicted_vehicle_actor_unique_id: i64) -> PlayerAuthInputVehicleInfo {
        PlayerAuthInputVehicleInfo{ vehicle_rotation_x, vehicle_rotation_z, predicted_vehicle_actor_unique_id }
    }

    pub fn read(stream: &mut Stream) -> PlayerAuthInputVehicleInfo {
        let vehicle_rotation_x = stream.get_l_float();
        let vehicle_rotation_z = stream.get_l_float();
        let predicted_vehicle_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);

        PlayerAuthInputVehicleInfo{ vehicle_rotation_x, vehicle_rotation_z, predicted_vehicle_actor_unique_id }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_float(self.vehicle_rotation_x);
        stream.put_l_float(self.vehicle_rotation_z);
        PacketSerializer::put_actor_unique_id(stream, self.predicted_vehicle_actor_unique_id);
    }
}