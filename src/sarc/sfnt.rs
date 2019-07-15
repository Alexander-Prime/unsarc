use byteorder::{ByteOrder, BE};

use crate::bom::ByteOrderMark;

use crate::Error;
use crate::Result;

pub const MAGIC: u32 = 0x53_46_4e_54; // SFNT

pub struct Sfnt {
    pub header_length: u16,
    pub reserved: Vec<u8>,
    pub names: Vec<u8>,
}

impl Sfnt {
    pub fn from_bytes(buf: &[u8], bom: ByteOrderMark) -> Result<Sfnt> {
        let m = BE::read_u32(&buf[..=0x03]);
        if m == MAGIC {
            println!("✔ SFNT");
            let header_length = bom.read_u16(&buf[0x04..=0x05]);
            let reserved = Vec::from(&buf[0x06..=0x07]);
            let names = Vec::from(&buf[0x08..]);

            Ok(Sfnt {
                header_length,
                reserved,
                names,
            })
        } else {
            Err(Error::BadMagic(m))
        }
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
