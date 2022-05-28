/// Void WAV audio file.
///
/// Other attributes: mono, pcm
pub struct VoidWAV {
    /// Unit: seconds.
    pub duration: f64,
    /// Unit: Hz. Lower than 64 may cause compatibility issues.
    pub sampling: u32,
}

impl Default for VoidWAV {
    fn default() -> Self {
        Self {
            duration: 5.0,
            sampling: 64,
        }
    }
}

impl VoidWAV {
    pub fn data(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        let len = (2.0 * self.duration * (self.sampling as f64)) as u32 + 44;
        ret.extend(b"RIFF");
        ret.extend((len - 8).to_le_bytes()); // little-endian please
        ret.extend(b"WAVEfmt\x20\x10\0\0\0\x01\0\x01\0");
        ret.extend(self.sampling.to_le_bytes());
        ret.extend((self.sampling * 2).to_le_bytes());
        ret.extend(b"\x02\0\x10\0\x64\x61\x74\x61");
        ret.extend((len - 44).to_le_bytes());
        ret.resize(len as usize, 0); // all of the wave data is 0x00
        ret
    }
}
