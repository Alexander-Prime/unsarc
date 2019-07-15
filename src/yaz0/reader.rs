use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

use byteorder::{ByteOrder, BE};

use crate::Error;
use crate::Result;

pub struct Yaz0Reader {
    input: Box<Read>,
    output: Vec<u8>,
    index: usize,
}

impl Yaz0Reader {
    pub fn open(path: &str) -> Result<Yaz0Reader> {
        File::open(path)
            .and_then(|mut file| file.seek(SeekFrom::Start(0x00)).map(|_| file))
            .map_err(|e| Error::ReadFailed(e))
            .and_then(|file| Self::from_reader(Box::from(file)))
    }

    pub fn from_reader(input: Box<Read>) -> Result<Yaz0Reader> {
        let mut buf: Vec<u8> = vec![0; 0x10];
        let mut input = BufReader::new(input);
        input
            .read(&mut buf[..])
            .map(|_| Yaz0Reader {
                input: Box::from(input),
                output: Vec::with_capacity(BE::read_u32(&buf[0x04..=0x07]) as usize),
                index: 0,
            })
            .map_err(|e| Error::ReadFailed(e))
    }
}

impl Read for Yaz0Reader {
    fn read(&mut self, dst: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        let mut chunk: Vec<u8> = vec![0; 4];

        let next_index = self.index + dst.len();

        while self.output.len() < next_index {
            self.input.read(&mut chunk[..=0x00]);

            // Header byte describes the next 8 chunks
            let mut chunk_head = chunk[0x00];

            for _ in 0x00..=0x07 {
                let use_rle = chunk_head & 0x80 == 0;

                if !use_rle {
                    // Copy byte directly
                    let new_len = self.output.len() + 1;
                    self.output.resize(new_len, 0);
                    self.input.read(&mut self.output[new_len - 1..new_len]);
                } else {
                    // Run length encoding
                    self.input.read(&mut chunk[..=0x01]);
                    let rewind: usize =
                        (((chunk[0x00] as usize & 0x0f) << 8) | chunk[0x01] as usize);
                    let mut size = (chunk[0x00] as usize & 0xf0) >> 4;
                    if size == 0 {
                        // Length uses an extra byte
                        self.input.read(&mut chunk[0x02..=0x02]);
                        size = chunk[0x02] as usize + 0x12;
                    } else {
                        size = size + 0x02;
                    }

                    let old_len = self.output.len();
                    self.output.resize(old_len + size, 0);

                    let copy_start = old_len - rewind;

                    slow_copy_within(&mut self.output[..], copy_start, size, old_len);
                }
                chunk_head <<= 1;
            }
        }

        dst.copy_from_slice(&self.output[self.index..next_index]);
        self.index = next_index;
        Ok(dst.len())
    }
}

// TODO See if real copy_within can work here
fn slow_copy_within(
    buf: &mut [u8],
    start: usize,
    length: usize,
    destination: usize,
) -> std::io::Result<usize> {
    for (dst, src) in (start..start + length)
        .into_iter()
        .enumerate()
        .map(|(i, src)| (i + destination, src))
    {
        buf[dst] = buf[src];
    }

    Ok(length)
}
