/// Blank PDF 1.3 file.
pub struct VoidPDF {
    /// Unit: px, on 72PPI.
    pub width: usize,
    /// Unit: px, on 72PPI.
    pub height: usize,
    pub page_count: usize,
}

impl Default for VoidPDF {
    fn default() -> Self {
        Self {
            width: 100,
            height: 200,
            page_count: 1,
        }
    }
}

impl VoidPDF {
    pub fn data(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        ret.extend(T_0);
        ret.append(&mut self.page_count.to_string().into_bytes());
        ret.extend(T_1);
        for _ in 0..self.page_count {
            ret.extend(T_2);
        }
        ret.extend(T_3);
        ret.append(&mut self.width.to_string().into_bytes());
        ret.push(b' ');
        ret.append(&mut self.height.to_string().into_bytes());
        ret.extend(T_4);
        ret
    }
}

const T_0: &[u8] = b"%PDF-1.3
1 0 obj
<<
/Type /Catalog
/Pages 2 0 R
>>
endobj
2 0 obj
<<
/Type /Pages
/Count ";
const T_1: &[u8] = b"
/Kids [";
const T_2: &[u8] = b"3 0 R ";
const T_3: &[u8] = b"]
>>
endobj
3 0 obj
<<
/Type /Page
/MediaBox [0 0 ";
const T_4: &[u8] = b"]
>>
endobj
xref
0 0
trailer
<<
/Root 1 0 R
>>
startxref
%%EOF";
