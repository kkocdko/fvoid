/// Blank SVG file.
pub struct VoidSVG {
    /// Unit: px.
    pub width: usize,
    /// Unit: px.
    pub height: usize,
    /// Follor CSS color format.
    pub color: String,
}

impl Default for VoidSVG {
    fn default() -> Self {
        Self {
            width: 100,
            height: 200,
            color: "white".into(),
        }
    }
}

impl VoidSVG {
    pub fn data(&self) -> Vec<u8> {
        let mut width = self.width.to_string().into_bytes();
        let mut height = self.height.to_string().into_bytes();
        let color = self.color.as_bytes();
        let mut ret = Vec::new();
        ret.extend(T_0);
        ret.append(&mut width);
        ret.extend(T_1);
        ret.append(&mut height);
        ret.extend(T_2);
        ret.extend(color);
        ret.extend(T_3);
        ret
    }
}

const T_0: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none" viewBox="0 0 100 100" width=""#;
const T_1: &[u8] = br#"" height=""#;
const T_2: &[u8] = br#""><rect width="100" height="100" fill=""#;
const T_3: &[u8] = br#""/></svg>"#;
