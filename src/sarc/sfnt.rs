use crate::bom::ByteOrderMark;

use super::magic::Magic;
use super::result::Result;

pub struct Sfnt {
    pub header_length: u16,
    pub reserved: Vec<u8>,
    pub names: Vec<u8>,
}

impl Sfnt {
    pub fn from_bytes(buf: &[u8], bom: ByteOrderMark) -> Result<Sfnt> {
        Magic::check(&buf[0x00..=0x03], "SFNT").map(|_| {
            let header_length = bom.read_u16(&buf[0x04..=0x05]);
            let reserved = Vec::from(&buf[0x06..=0x07]);
            let names = Vec::from(&buf[0x08..]);

            Sfnt {
                header_length,
                reserved,
                names,
            }
        })
    }

    pub fn read_name(&self, offset: usize) -> String {
        let slice = &self.names[offset..];
        let end = match slice.iter().position(|&byte| byte == 0) {
            Some(end) => end,
            None => self.names.len(),
        };
        String::from_utf8(Vec::from(&slice[..end])).unwrap_or(String::default())
    }
}
