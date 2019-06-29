mod error;
mod header;
mod magic;
mod node;
mod result;
mod sfat;
mod sfnt;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::vec::Vec;

use self::error::Error;
use self::header::Header;
use self::result::Result;
use self::sfat::Sfat;
use self::sfnt::Sfnt;

pub struct SarcFile {
    pub input: Box<Read>,
    pub header: Header,
    pub sfat: Sfat,
    pub sfnt: Sfnt,
}

impl SarcFile {
    pub fn open(path: &str) -> Result<SarcFile> {
        match File::open(path).and_then(|mut file| file.seek(SeekFrom::Start(0x00)).map(|_| file)) {
            Ok(file) => Self::from_reader(Box::from(file)),
            Err(e) => Err(Error::ReadFailed(e)),
        }
    }

    fn from_reader(mut input: Box<Read>) -> Result<SarcFile> {
        // Don't need a BufReader here, it would only save at best one read() call
        let mut buf: Vec<u8> = Vec::with_capacity(0x14);

        if let Err(e) = input.read(&mut buf[0x00..0x13]) {
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

        let sfnt = match Sfnt::from_bytes(&buf[sfat.length()..], header.bom) {
            Ok(sfnt) => sfnt,
            Err(e) => return Err(e),
        };

        Ok(SarcFile {
            input,
            header,
            sfat,
            sfnt,
        })
    }
}
