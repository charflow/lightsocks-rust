use bytes::Bytes;
use std::fmt;

pub trait Cipher {
    fn encode(input: &Bytes) -> Bytes;
    fn decode(input: &Bytes) -> Bytes;
}

pub trait SecureSocket {
    fn encode_write(&mut self, bs: &Bytes) -> Result<(), Error>;
    fn decode_read(&self) -> Bytes;
}

#[derive(Debug, Clone)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error {
    pub fn new(err: String) -> Error {
        Error(err)
    }
}
