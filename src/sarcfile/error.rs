pub enum Error {
    ReadFailed(std::io::Error),
    BadEncoding(std::str::Utf8Error),
    WrongType(String),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ReadFailed(e) => writeln!(f, "Read failed!").and_then(|_| write!(f, "{}", e)),
            Error::BadEncoding(e) => writeln!(f, "Bad encoding!").and_then(|_| write!(f, "{}", e)),
            Error::WrongType(t) => writeln!(f, "Wrong type {}", t),
        }
    }
}
