/// Void MP3 audio file.
///
/// Other attributes: 8000 Hz, mono, s16p, 8 kb/s
pub struct VoidMP3 {
    /// Unit: seconds.
    pub duration: f64,
}

impl Default for VoidMP3 {
    fn default() -> Self {
        Self { duration: 5.0 }
    }
}

impl VoidMP3 {
    pub fn data(&self) -> Vec<u8> {
        let mut ret = Vec::new();

        ret.extend(T_0);
        ret.resize(ret.len() + 23, 0);

        // 72 seconds => 1000 blocks
        let block_count = (self.duration * 1000.0 / 72.0).round() as usize;

        for _ in 0..block_count {
            ret.extend(T_1);
            ret.resize(ret.len() + 59, 0x55);
        }

        ret
    }
}

#[rustfmt::skip]
const T_0: &[u8] = &[
0x49, 0x44, 0x33, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x54, 0x53, 0x53, 0x45, 0x00, 0x00,
0x00, 0x0F, 0x00, 0x00, 0x03, 0x4C,
// then append `0x00` 23 times
];
#[rustfmt::skip]
const T_1: &[u8] = &[
0xFF, 0xE3, 0x18, 0xC4, 0xC4, 0x00, 0x00, 0x03, 0x48, 0x00, 0x00, 0x00, 0x00,
// then append `0x55` 59 times
];

/*
ffmpeg -f lavfi -i anullsrc=r=8000:cl=mono -t 10 -b:a 8k sample.mp3

https://hexed.it

let s='',d=`
0xFF, 0xE3, 0x18, 0xC4, 0xC4, 0x00, 0x00, 0x03, 0x48, 0x00, 0x00, 0x00
`.split(',').map(v=>v.trim());
for(let i=0;i<d.length;i++)s+=d[i]+','+(i%16==15?'\n':' ');
console.log(s);
*/
