use super::traits::Cipher;
use super::Bytes;
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

pub struct Chacha20Poly1305Wrapper {
    #[allow(dead_code)]
    password: String,
    cipher: ChaCha20Poly1305,
}

impl Chacha20Poly1305Wrapper {
    pub fn new_cipher(password: &str) -> Chacha20Poly1305Wrapper {
        let md5digest = md5(password.to_string());
        let key = Key::from_slice(md5digest.as_bytes());
        let cipher = ChaCha20Poly1305::new(key);
        Chacha20Poly1305Wrapper {
            password: password.to_owned(),
            cipher,
        }
    }

    pub fn encrypt(&self, nonce: u64, plaintext: &[u8]) -> Vec<u8> {
        let padded_nonce = format!("{:0>12}", nonce % 1_000_000_000_000); // 取余保证不超过12位
        let nonce = Nonce::from_slice(padded_nonce.as_bytes());
        self.cipher
            .encrypt(nonce, plaintext)
            .expect("encryption failure!")
    }

    pub fn decrypt(&self, nonce: u64, ciphertext: &[u8]) -> Vec<u8> {
        let padded_nonce = format!("{:0>12}", nonce % 1_000_000_000_000); // 取余保证不超过12位
        let nonce = Nonce::from_slice(padded_nonce.as_bytes());
        self.cipher
            .decrypt(nonce, ciphertext)
            .expect("decryption failure!")
    }
}

const NONCE: u64 = 1314;
impl Cipher for Chacha20Poly1305Wrapper {
    fn encrypt(&self, input: &Bytes) -> Vec<u8> {
        self.encrypt(NONCE, input)
    }

    fn decrypt(&self, input: &Bytes) -> Vec<u8> {
        self.decrypt(NONCE, input)
    }
}

pub fn md5(content: String) -> String {
    let digest = md5::compute(content.as_bytes());
    format!("{:x}", digest)
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_cipher() {
        let cipher = Chacha20Poly1305Wrapper::new_cipher("hello, lightsocks");
        let text = b"plaintext message";
        let ciphertext = cipher.encrypt(666, text);
        let plaintext = cipher.decrypt(666, &ciphertext);
        assert_eq!(plaintext.len(), text.len());
        assert_eq!(&plaintext, text);
    }
}
