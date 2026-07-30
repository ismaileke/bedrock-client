use crate::utils::color::Color;

#[derive(serde::Serialize, Debug)]
pub struct MapDecoration {
    pub icon: u8,
    pub rotation: u8,
    pub x_offset: u8,
    pub y_offset: u8,
    pub label: String,
    pub color: Color
}
