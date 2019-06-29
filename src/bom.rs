use std::convert::From;

use byteorder::{BigEndian, ByteOrder, LittleEndian};

#[derive(Clone, Copy)]
pub enum ByteOrderMark {
    Big,
    Little,
}

impl ByteOrderMark {
    pub fn read_u16(&self, buf: &[u8]) -> u16 {
        match self {
            ByteOrderMark::Big => BigEndian::read_u16(buf),
            ByteOrderMark::Little => LittleEndian::read_u16(buf),
        }
    }

    pub fn read_u32(&self, buf: &[u8]) -> u32 {
        match self {
            ByteOrderMark::Big => BigEndian::read_u32(buf),
            ByteOrderMark::Little => LittleEndian::read_u32(buf),
        }
    }
}

impl From<&[u8]> for ByteOrderMark {
    fn from(buf: &[u8]) -> ByteOrderMark {
        match BigEndian::read_u16(buf) {
            0xfffe => ByteOrderMark::Little,
            _ => ByteOrderMark::Big,
        }
    }
}
