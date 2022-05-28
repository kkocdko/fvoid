/// Void M3U (M3U8) playlist file.
pub struct VoidM3U {
    /// Unit: seconds.
    pub duration: f64,
    /// The first and only filename.
    pub filename: String,
}

impl Default for VoidM3U {
    fn default() -> Self {
        Self {
            duration: 5.0,
            filename: "".into(),
        }
    }
}

impl VoidM3U {
    pub fn data(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        let duration = self.duration.to_string().into_bytes();
        ret.extend(b"#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-TARGETDURATION:");
        ret.extend(&duration);
        ret.extend(b"\n#EXT-X-MEDIA-SEQUENCE:0\n#EXTINF:");
        ret.extend(&duration);
        ret.extend(b",\n");
        ret.extend(self.filename.as_bytes());
        ret.extend(b"\n#EXT-X-ENDLIST\n");
        ret
    }
}
