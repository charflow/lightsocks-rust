use super::Bytes;
use std::fmt;

pub trait Cipher {
    fn encrypt(&self, input: &Bytes) -> Vec<u8>;
    fn decrypt(&self, input: &Bytes) -> Vec<u8>;
}

pub trait SecureSocket {
    fn encode_write(&mut self, bs: &Bytes) -> Result<(), Error>;
    fn decode_read(&self) -> Vec<u8>;
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
