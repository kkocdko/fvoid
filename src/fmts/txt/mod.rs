/// Plain text file.
pub struct VoidTXT {
    /// Each byte will be filled to this.
    pub content: u8,
    /// Unit: bytes.
    pub size: usize,
}

#[allow(clippy::derivable_impls)]
impl Default for VoidTXT {
    fn default() -> Self {
        Self {
            content: 0,
            size: 0,
        }
    }
}

impl VoidTXT {
    pub fn data(&self) -> Vec<u8> {
        vec![self.content; self.size]
    }
}
