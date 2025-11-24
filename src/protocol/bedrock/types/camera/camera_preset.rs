use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_preset_aim_assist::CameraPresetAimAssist;

#[derive(serde::Serialize, Debug)]
pub struct CameraPreset {
    pub name: String,
    pub parent: String,
    pub x_position: Option<f32>,
    pub y_position: Option<f32>,
    pub z_position: Option<f32>,
    pub pitch: Option<f32>,
    pub yaw: Option<f32>,
    pub rotation_seed: Option<f32>,
    pub snap_to_target: Option<bool>,
    pub horizontal_rotation_limit: Option<Vec<f32>>,
    pub vertical_rotation_limit: Option<Vec<f32>>,
    pub continue_targeting: Option<bool>,
    pub block_listening_radius: Option<f32>,
    pub view_offset: Option<Vec<f32>>,
    pub entity_offset: Option<Vec<f32>>,
    pub radius: Option<f32>,
    pub yaw_limit_min: Option<f32>,
    pub yaw_limit_max: Option<f32>,
    pub audio_listener_type: Option<u8>,
    pub player_effects: Option<bool>,
    pub aim_assist: Option<CameraPresetAimAssist>,
    pub control_scheme: Option<u8>, //see types/control_scheme.rs
}

impl CameraPreset {
    pub const AUDIO_LISTENER_TYPE_CAMERA: u8 = 0;
    pub const AUDIO_LISTENER_TYPE_PLAYER: u8 = 1;

    pub fn new(
        name: String,
        parent: String,
        x_position: Option<f32>,
        y_position: Option<f32>,
        z_position: Option<f32>,
        pitch: Option<f32>,
        yaw: Option<f32>,
        rotation_seed: Option<f32>,
        snap_to_target: Option<bool>,
        horizontal_rotation_limit: Option<Vec<f32>>,
        vertical_rotation_limit: Option<Vec<f32>>,
        continue_targeting: Option<bool>,
        block_listening_radius: Option<f32>,
        view_offset: Option<Vec<f32>>,
        entity_offset: Option<Vec<f32>>,
        radius: Option<f32>,
        yaw_limit_min: Option<f32>,
        yaw_limit_max: Option<f32>,
        audio_listener_type: Option<u8>,
        player_effects: Option<bool>,
        aim_assist: Option<CameraPresetAimAssist>,
        control_scheme: Option<u8>,
    ) -> CameraPreset {
        CameraPreset{
            name,
            parent,
            x_position,
            y_position,
            z_position,
            pitch,
            yaw,
            rotation_seed,
            snap_to_target,
            horizontal_rotation_limit,
            vertical_rotation_limit,
            continue_targeting,
            block_listening_radius,
            view_offset,
            entity_offset,
            radius,
            yaw_limit_min,
            yaw_limit_max,
            audio_listener_type,
            player_effects,
            aim_assist,
            control_scheme
        }
    }

    pub fn read(stream: &mut Stream) -> CameraPreset {
        let name = PacketSerializer::get_string(stream);
        let parent = PacketSerializer::get_string(stream);
        let x_position = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let y_position = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let z_position = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let pitch = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let yaw = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let rotation_seed = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let snap_to_target = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let horizontal_rotation_limit = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector2(s));
        let vertical_rotation_limit = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector2(s));
        let continue_targeting = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let block_listening_radius = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let view_offset = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector2(s));
        let entity_offset = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let radius = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let yaw_limit_min = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let yaw_limit_max = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let audio_listener_type = PacketSerializer::read_optional(stream, |s| s.get_byte());
        let player_effects = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let aim_assist = PacketSerializer::read_optional(stream, |s| CameraPresetAimAssist::read(s));
        let control_scheme = PacketSerializer::read_optional(stream, |s| s.get_byte());

        CameraPreset{
            name,
            parent,
            x_position,
            y_position,
            z_position,
            pitch,
            yaw,
            rotation_seed,
            snap_to_target,
            horizontal_rotation_limit,
            vertical_rotation_limit,
            continue_targeting,
            block_listening_radius,
            view_offset,
            entity_offset,
            radius,
            yaw_limit_min,
            yaw_limit_max,
            audio_listener_type,
            player_effects,
            aim_assist,
            control_scheme
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        PacketSerializer::put_string(stream, self.parent.clone());
        PacketSerializer::write_optional(stream, &self.x_position, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.y_position, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.z_position, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.pitch, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.yaw, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.rotation_seed, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.snap_to_target, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.horizontal_rotation_limit, |s, v| PacketSerializer::put_vector2(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.vertical_rotation_limit, |s, v| PacketSerializer::put_vector2(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.continue_targeting, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.block_listening_radius, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.view_offset, |s, v| PacketSerializer::put_vector2(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.entity_offset, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.radius, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.yaw_limit_min, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.yaw_limit_max, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.audio_listener_type, |s, v| s.put_byte(*v));
        PacketSerializer::write_optional(stream, &self.player_effects, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.aim_assist, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.control_scheme, |s, v| s.put_byte(*v));
    }
}