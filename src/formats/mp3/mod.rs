/// Slient MP3 audio file
pub struct VoidMP3 {
    pub length: f64, // seconds
}

impl Default for VoidMP3 {
    fn default() -> Self {
        Self { length: 5.0 }
    }
}

impl VoidMP3 {
    pub fn data(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        ret.extend(T_0);

        // 72 seconds => 1000 + 1 blocks
        let block_count = (self.length * 1000.0 / 72.0).round() as usize + 1;

        ret.extend([0, 0, 0, 0]);
        // ret.push(block_count as u8); // todo: extend bits

        ret.extend(T_1);

        for _ in 0..block_count {
            ret.extend(T_2);
            for _ in 0..59 {
                ret.push(0x55);
            }
        }

        ret
    }
}

#[rustfmt::skip]
const T_0:&[u8]=&[
0x49, 0x44, 0x33, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x54, 0x53, 0x53, 0x45, 0x00, 0x00,
0x00, 0x0F, 0x00, 0x00, 0x03, 0x4C, 0x61, 0x76, 0x66, 0x35, 0x39, 0x2E, 0x31, 0x36, 0x2E, 0x31,
0x30, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xE3, 0x38,
0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x6E, 0x66, 0x6F, 0x00, 0x00,
0x00, 0x0F, /*    block count   */ ];

#[rustfmt::skip]
const T_1:&[u8]=&[
                                    0x00, 0x00, 0x01, 0xB0, 0x00, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xD5, 0xD5, 0xD5, 0xD5,
0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5,
0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xD5, 0xFF, 0xFF, 0xFF,
0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
0x00, 0x00, 0x4C, 0x61, 0x76, 0x63, 0x35, 0x39, 0x2E, 0x31, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x02, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x01, 0xB0, 0xF7, 0x0A, 0xAA, 0xC3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 
];

const T_2: &[u8] = &[
    0xFF, 0xE3, 0x18, 0xC4, 0xC4, 0x00, 0x00, 0x03, 0x48, 0x00, 0x00, 0x00,
    0x00, // then append `0x55` 59 times
];

/*
https://hexed.it
let s='',d=`
0xFF, 0xE3, 0x18, 0xC4, 0xC4, 0x00, 0x00, 0x03, 0x48, 0x00, 0x00, 0x00
`.split(',').map(v=>v.trim());
for(let i=0;i<d.length;i++)s+=d[i]+','+(i%16==15?'\n':' ');
console.log(s);
*/
