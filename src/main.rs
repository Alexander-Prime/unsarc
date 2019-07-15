mod bom;
mod bufmt;
mod sarc;
mod yaz0;

use std::io::{Read, Seek, SeekFrom};

use byteorder::{ByteOrder, BE};

use sarc::reader::SarcReader;
use yaz0::reader::Yaz0Reader;

pub enum Error {
    ReadFailed(std::io::Error),
    BadMagic(u32),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ReadFailed(e) => writeln!(f, "Read failed!").and_then(|_| write!(f, "{:?}", e)),
            Error::BadMagic(m) => write!(f, "Bad magic: ")
                .and_then(|_| write!(f, "{} [{:08x}]", Magic(&m.to_be_bytes()), m)),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for path in &args[1..] {
        if let Err(e) = std::fs::File::open(path)
            .map_err(|e| Error::ReadFailed(e))
            .and_then(|mut file| {
                let mut buf = vec![0; 4];
                file.read(&mut buf[..]);
                file.seek(SeekFrom::Start(0x00));

                println!("{}", Magic(&buf[..=0x03]));

                let m = BE::read_u32(&buf[..=0x03]);
                if m == yaz0::MAGIC {
                    Yaz0Reader::from_reader(Box::from(file))
                        .and_then(|yaz0| SarcReader::from_reader(Box::from(yaz0)))
                } else if m == sarc::MAGIC {
                    SarcReader::from_reader(Box::from(file))
                } else {
                    Err(Error::BadMagic(m))
                }
            })
            .map(|file| {
                for node in file.nodes() {
                    println!("{}", node);
                }
            })
        {
            println!("{:?}", e)
        }
    }
}

struct Magic<'a>(&'a [u8]);

impl<'a> std::fmt::Display for Magic<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ ")
            .and_then(|_| {
                self.0
                    .into_iter()
                    .map(|&byte| write!(f, "{} ", byte as char))
                    .collect::<std::fmt::Result>()
            })
            .and_then(|_| write!(f, "]"))
    }
}
