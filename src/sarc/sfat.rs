use crate::bom::ByteOrderMark;

use super::magic::Magic;
use super::node::Node;
use super::result::Result;

const NODE_SIZE: usize = 0x10;

pub struct Sfat {
    pub header_length: u16,
    pub node_count: u16,
    pub hash_key: u32,
    pub nodes: Vec<Node>,
}

impl Sfat {
    pub fn from_bytes(buf: &[u8], bom: ByteOrderMark) -> Result<Sfat> {
        Magic::check(&buf[0x00..=0x03], "SFAT").map(|_| {
            let header_length = bom.read_u16(&buf[0x04..=0x05]);
            let node_count = bom.read_u16(&buf[0x06..=0x07]);
            let hash_key = bom.read_u32(&buf[0x08..=0x0b]);
            let mut nodes = Vec::with_capacity(node_count as usize);

            for node_index in 0..node_count as usize {
                let start = header_length as usize + (node_index * NODE_SIZE);
                let end = start + NODE_SIZE;
                nodes.push(Node::from_bytes(&buf[start..end], bom));
            }

            Sfat {
                header_length,
                node_count,
                hash_key,
                nodes,
            }
        })
    }

    pub fn length(&self) -> usize {
        self.header_length as usize + (self.node_count as usize * NODE_SIZE)
    }
}