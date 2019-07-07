use crate::bom::ByteOrderMark;

use super::magic::Magic;
use super::result::Result;

pub struct Header {
    pub bom: ByteOrderMark,
    pub length: u16,
    pub file_size: u32,
    pub start_offset: u32,
}

impl Header {
    pub fn from_bytes(buf: &[u8]) -> Result<Header> {
        Magic::check(&buf[0x00..=0x03], "SARC").map(|_| {
            let bom = ByteOrderMark::from(&buf[0x06..=0x07]);
            Header {
                bom,
                length: bom.read_u16(&buf[0x04..=0x05]),
                file_size: bom.read_u32(&buf[0x08..=0x0b]),
                start_offset: bom.read_u32(&buf[0x0c..=0x0f]),
            }
        })
    }
}
