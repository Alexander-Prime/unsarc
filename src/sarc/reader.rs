use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::vec::Vec;

use crate::Error;
use crate::Result;

use super::header::Header;
use super::sfat::Sfat;
use super::sfnt::Sfnt;

pub struct SarcReader {
    pub input: Box<Read>,
    pub header: Header,
    pub sfat: Sfat,
    pub sfnt: Sfnt,
}

impl SarcReader {
    pub fn open(path: &str) -> Result<SarcReader> {
        File::open(path)
            .and_then(|mut file| file.seek(SeekFrom::Start(0x00)).map(|_| file))
            .map_err(|e| Error::ReadFailed(e))
            .and_then(|file| Self::from_reader(Box::from(file)))
    }

    pub fn from_reader(mut input: Box<Read>) -> Result<SarcReader> {
        // Don't need a BufReader here, it would only save at best one read() call
        let mut buf: Vec<u8> = vec![0; 0x14];

        if let Err(e) = input.read(&mut buf[0x00..=0x13]) {
            return Err(Error::ReadFailed(e));
        }

        let header = match Header::from_bytes(&buf[0x00..=0x13]) {
            Ok(header) => header,
            Err(e) => return Err(e),
        };

        buf.resize(header.start_offset as usize, 0);

        if let Err(e) = input.read(&mut buf[0x14..]) {
            return Err(Error::ReadFailed(e));
        }

        let sfat = match Sfat::from_bytes(&buf[0x14..], header.bom) {
            Ok(sfat) => sfat,
            Err(e) => return Err(e),
        };

        let sfnt_start = header.length as usize + sfat.length();
        let sfnt = match Sfnt::from_bytes(&buf[sfnt_start..], header.bom) {
            Ok(sfnt) => sfnt,
            Err(e) => return Err(e),
        };

        Ok(SarcReader {
            input,
            header,
            sfat,
            sfnt,
        })
    }

    pub fn nodes(&self) -> NodeIterator {
        NodeIterator {
            sarc: self,
            index: 0,
        }
    }
}

pub struct NodeIterator<'a> {
    sarc: &'a SarcReader,
    index: usize,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.sarc.sfat.node_count as usize {
            None
        } else {
            let sfat = &self.sarc.sfat;
            let sfnt = &self.sarc.sfnt;
            let name = sfnt.read_name((sfat.nodes[self.index].filename_offset * 4) as usize);
            self.index += 1;
            Some(name)
        }
    }
}
