use byteorder::{ByteOrder, BE};

use crate::bom::ByteOrderMark;

use crate::Error;
use crate::Result;

pub struct Header {
    pub bom: ByteOrderMark,
    pub length: u16,
    pub file_size: u32,
    pub start_offset: u32,
}

impl Header {
    pub fn from_bytes(buf: &[u8]) -> Result<Header> {
        let m = BE::read_u32(&buf[..=0x03]);
        if m == super::MAGIC {
            println!("✔ SARC");
            let bom = ByteOrderMark::from(&buf[0x06..=0x07]);
            Ok(Header {
                bom,
                length: bom.read_u16(&buf[0x04..=0x05]),
                file_size: bom.read_u32(&buf[0x08..=0x0b]),
                start_offset: bom.read_u32(&buf[0x0c..=0x0f]),
            })
        } else {
            Err(Error::BadMagic(m))
        }
    }
}
