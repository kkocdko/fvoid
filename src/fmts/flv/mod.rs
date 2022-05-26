/// Blank FLV video file.
pub struct VoidFLV {
    /// Unit: seconds.
    pub duration: f64,
    /// Frames per second, affects jump accuracy. Suggested values: 1, 10, 30.
    pub fps: f64,
    // Other info: resolution = 4x4, color = black
}

impl Default for VoidFLV {
    fn default() -> Self {
        Self {
            duration: 5.0,
            fps: 1.0,
        }
    }
}

impl VoidFLV {
    pub fn data(&self) -> Vec<u8> {
        let mut ret = Vec::new();

        ret.extend(T_0);
        ret.extend(self.duration.to_be_bytes());

        ret.extend(T_1);
        ret.extend(self.fps.to_be_bytes());

        ret.extend(T_2);
        let filesize_idx = ret.len();
        ret.extend(0f64.to_be_bytes());

        ret.extend(T_3);

        let frame_ms = (1000.0 / self.fps).round() as u32;
        let frame_count = (self.duration * 1000.0 / (frame_ms as f64)).round() as u32;
        for i in 0..frame_count {
            let mut timestamp = (i * frame_ms).to_be_bytes();
            timestamp.rotate_left(1);

            ret.extend(T_4);
            ret.extend(timestamp);
            ret.extend(T_5);
        }

        let filesize = &(ret.len() as f64).to_be_bytes()[..];
        ret[filesize_idx..filesize_idx + filesize.len()].copy_from_slice(filesize);

        ret
    }
}

const T_0: &[u8] = include_bytes!("t0.bin") /* duration (double, 8 bytes) */ ;
#[rustfmt::skip]
const T_1: &[u8] = &[
                                                                                    0x00, 0x09,
0x66, 0x72, 0x61, 0x6D, 0x65, 0x72, 0x61, 0x74, 0x65, 0x00, /*    framerate (double, 8 bytes)
        */ 
];
#[rustfmt::skip]
const T_2: &[u8] = &[
            0x00, 0x08, 0x66, 0x69, 0x6C, 0x65, 0x73, 0x69, 0x7A, 0x65, 0x00, /* filesize
 (double, 8 bytes)        */
];
#[rustfmt::skip]
const T_3: &[u8] = &[         0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0xC3];
const T_4: &[u8] = &[
    0x09, 0x00, 0x00,
    0x0F, // timestamp, 4 bytes. 8h = 288e5ms = [01 b7 74 00], move `01` to end, => [b7 74 00 01]
];
const T_5: &[u8] = &[
    0x00, 0x00, 0x00, /* 0x12 always keyframe */ 0x12, 0x00, 0x00, 0x84, 0x00, 0x02, 0x02,
    0x11, 0x26, 0x20, 0x20, 0x20, 0x21, 0xFF, 0xFE, 0x00, 0x00, 0x00, 0x1A,
];

/*
ffmpeg -y -f lavfi -i nullsrc=size=1x1:rate=1:duration=50,lutrgb=0:0:0 flv50.flv
thanks to jianshu.com/p/7ffaec7b3be6
*/
