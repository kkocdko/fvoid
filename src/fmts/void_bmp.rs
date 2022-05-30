/// Void BMP image file.
///
/// Other attributes: color = black, palette = 1bit
pub struct VoidBMP {
    /// Unit: px.
    pub width: u32,
    /// Unit: px.
    pub height: u32,
}

impl Default for VoidBMP {
    fn default() -> Self {
        Self {
            width: 100,
            height: 200,
        }
    }
}

impl VoidBMP {
    pub fn data(&self) -> Vec<u8> {
        let mut ret = Vec::new();

        let data_size = self.height * (self.width + 31) / 32 * 4;
        let file_size = data_size + 62;
        ret.extend(b"BM");
        ret.extend(file_size.to_le_bytes());
        ret.extend(b"\0\0\0\0\x3e\0\0\0\x28\0\0\0");
        ret.extend(self.width.to_le_bytes());
        ret.extend(self.height.to_le_bytes());
        ret.extend(b"\x01\0\x01\0\0\0\0\0");
        ret.extend(data_size.to_le_bytes());
        ret.resize(58, 0);
        ret.extend(b"\xff\xff\xff");
        ret.resize(file_size as usize, 0);

        ret
    }
}
