/// Void MP4 video file.
///
/// Other attributes: resolution = 4x4, color = black
pub struct VoidMP4 {
    /// Unit: seconds.
    pub duration: f64,
    /// Frames per second, affects jump accuracy. Suggested values: 1, 5, 30.
    pub fps: f64,
}

impl Default for VoidMP4 {
    fn default() -> Self {
        Self {
            duration: 5.0,
            fps: 1.0,
        }
    }
}

impl VoidMP4 {
    pub fn data(&self) -> Vec<u8> {
        todo!()
    }
}

/*
https://gpac.github.io/mp4box.js/test/filereader.html
./tools/ffmpeg -y -f lavfi -i nullsrc=size=4x4:rate=1:duration=9,lutrgb=0:0:0 -force_key_frames "expr:gte(t,n_forced*1)" -crf 50 d9r1.mp4
*/
