use block_padding::generic_array::GenericArray;
use chacha20poly1305::{
    aead::{Aead, OsRng},
    consts::{U12, U32},
    AeadCore, ChaCha20Poly1305, KeyInit,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Key {
    key: Vec<u8>,
    nonce: Vec<u8>,
}

pub struct Cipher {
    cipher: ChaCha20Poly1305,
    key: GenericArray<u8, U32>,
    nonce: GenericArray<u8, U12>,
}

impl Default for Cipher {
    fn default() -> Self {
        Self::new()
    }
}

impl Cipher {
    pub fn new() -> Self {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        Cipher { cipher, key, nonce }
    }

    pub fn with_key(key: Vec<u8>) -> Self {
        let key = GenericArray::clone_from_slice(&key);
        let cipher = ChaCha20Poly1305::new_from_slice(&key).unwrap();
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        Cipher { cipher, key, nonce }
    }

    pub fn load(key_struct: &Key) -> Self {
        let key = GenericArray::clone_from_slice(&key_struct.key);
        let cipher = ChaCha20Poly1305::new_from_slice(&key_struct.key).unwrap();
        let nonce = GenericArray::clone_from_slice(&key_struct.nonce);
        Cipher { cipher, key, nonce }
    }

    pub fn get_key_struct(&self) -> Key {
        Key {
            key: self.key.to_vec(),
            nonce: self.nonce.to_vec(),
        }
    }

    pub fn encrypt(&self, buf: &[u8]) -> Vec<u8> {
        self.cipher.encrypt(&self.nonce, buf).unwrap()
    }

    pub fn decrypt(&self, buf: &[u8]) -> Vec<u8> {
        self.cipher.decrypt(&self.nonce, buf).unwrap()
    }
}

#[cfg(test)]
mod test {
    use chacha20poly1305::aead::rand_core::RngCore;

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let cipher = Cipher::new();
        let mut msg = vec![0u8; 16];
        OsRng.fill_bytes(&mut msg);

        assert_eq!(msg, cipher.decrypt(&cipher.encrypt(&msg)));
    }
}
