use super::Bytes;

pub trait Cipher {
    fn encrypt(&self, input: &Bytes) -> Vec<u8>;
    fn decrypt(&self, input: &Bytes) -> Vec<u8>;
}

pub trait SecureTcpStream {
    fn encrypt_write(&mut self, bs: &[u8]) -> Result<(), std::io::Error>;
    fn decrypt_read(&self) -> Vec<u8>;
}
