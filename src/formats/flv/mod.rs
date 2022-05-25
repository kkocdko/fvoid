/// FLV video file
pub struct VoidFLV {
    pub length: f64,
}

impl Default for VoidFLV {
    fn default() -> Self {
        Self { length: 5.0 }
    }
}

impl VoidFLV {
    pub fn data(&self) -> Vec<u8> {
        Vec::new()
    }
}
