use crate::bom::ByteOrderMark;

pub struct Node {
    pub filename_hash: u32,
    pub flags: u16,
    pub filename_offset: u16,
    pub start_offset: u32,
    pub end_offset: u32,
}

impl Node {
    pub fn from_bytes(buf: &[u8], bom: ByteOrderMark) -> Node {
        let file_attributes = bom.read_u32(&buf[0x04..=0x07]);
        Node {
            filename_hash: bom.read_u32(&buf[0x00..=0x03]),
            flags: (file_attributes >> 16) as u16,
            filename_offset: (file_attributes & 0xffff) as u16,
            start_offset: bom.read_u32(&buf[0x08..=0x0b]),
            end_offset: bom.read_u32(&buf[0x0c..=0x0f]),
        }
    }
}
