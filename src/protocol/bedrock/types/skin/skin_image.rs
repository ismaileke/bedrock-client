#[derive(serde::Serialize, Debug)]
pub struct SkinImage {
    height: u32,
    width: u32,
    data: String
}

impl SkinImage {
    pub fn new(height: u32, width: u32, data: String) -> SkinImage {
        SkinImage { height, width, data }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn data(&self) -> String {
        self.data.clone()
    }
}
