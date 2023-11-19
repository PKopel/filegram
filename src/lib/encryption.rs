use std::fs::File;

use block_padding::generic_array::GenericArray;
use chacha20poly1305::{
    aead::{Aead, OsRng},
    consts::U12,
    AeadCore, ChaCha20Poly1305, KeyInit,
};
use serde::{Deserialize, Serialize};

pub struct Cipher {
    cipher: ChaCha20Poly1305,
    nonce: GenericArray<u8, U12>,
}

#[derive(Serialize, Deserialize)]
struct Key {
    key: Vec<u8>,
    nonce: Vec<u8>,
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
        let key_struct = Key {
            key: key.to_vec(),
            nonce: nonce.to_vec(),
        };
        let key_file = File::create("filegram.key").unwrap();
        serde_json::to_writer(key_file, &key_struct).unwrap();
        Cipher { cipher, nonce }
    }

    pub fn with_key(key: Vec<u8>) -> Self {
        let cipher = ChaCha20Poly1305::new_from_slice(&key).unwrap();
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let key_struct = Key {
            key,
            nonce: nonce.to_vec(),
        };
        let key_file = File::open("filegram.key").unwrap();
        serde_json::to_writer(key_file, &key_struct).unwrap();
        Cipher { cipher, nonce }
    }

    pub fn load(file: File) -> Self {
        let key_struct: Key = serde_json::from_reader(file).unwrap();
        let cipher = ChaCha20Poly1305::new_from_slice(&key_struct.key).unwrap();
        let nonce = GenericArray::clone_from_slice(&key_struct.nonce);
        Cipher { cipher, nonce }
    }

    pub fn encrypt(&self, buf: &[u8]) -> Vec<u8> {
        self.cipher.encrypt(&self.nonce, buf).unwrap()
    }

    pub fn decrypt(&self, buf: &[u8]) -> Vec<u8> {
        self.cipher.decrypt(&self.nonce, buf).unwrap()
    }
}
