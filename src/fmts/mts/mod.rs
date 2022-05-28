/// Void MPEG-TS video file.
///
/// Other attributes: resolution = 4x4, color = black
pub struct VoidMTS {
    /// Unit: seconds.
    pub duration: f64,
    /// Frames per second, affects jump accuracy. Suggested values: 1, 5, 30.
    pub fps: f64,
}

impl Default for VoidMTS {
    fn default() -> Self {
        Self {
            duration: 5.0,
            fps: 1.0,
        }
    }
}

impl VoidMTS {
    pub fn data(&self) -> Vec<u8> {
        todo!();
    }
}

/*
.m3u8 = application/x-mpegURL
.ts = video/MP2T
https://en.wikipedia.org/wiki/MPEG_transport_stream
*/
