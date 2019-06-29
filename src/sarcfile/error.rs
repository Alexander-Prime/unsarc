pub enum Error {
    ReadFailed(std::io::Error),
    BadEncoding(std::str::Utf8Error),
    WrongType(String),
}
