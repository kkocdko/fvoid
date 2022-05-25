/// Plain text file
pub struct VoidTXT {
    pub content: u8,
    pub size: usize,
}

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
