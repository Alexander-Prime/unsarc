use super::error::Error;
use super::result::Result;

pub struct Magic(String);

impl Magic {
    pub fn check(buf: &[u8], target: &str) -> Result<Magic> {
        match std::str::from_utf8(&buf[0x00..=0x03]) {
            Ok(m) if m == target => Ok(Magic(m.to_string())),
            Ok(m) => Err(Error::WrongType(m.to_string())),
            Err(e) => Err(Error::BadEncoding(e)),
        }
    }
}
