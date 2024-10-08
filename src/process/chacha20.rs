use base64::prelude::*;
use std::{fs, path::Path};

use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

use crate::Chacha;

pub trait EnDeCode {
    fn new(key: &[u8]) -> Self;
    fn readkey(path: &Path) -> anyhow::Result<Vec<u8>>;
    fn genkey() -> anyhow::Result<Vec<u8>>;
    fn encode(&self, data: &[u8]) -> anyhow::Result<Vec<u8>>;
    fn decode(&self, data: &[u8]) -> anyhow::Result<Vec<u8>>;
}

impl EnDeCode for Chacha {
    fn new(key: &[u8]) -> Self {
        Self { key: key.to_vec() }
    }

    fn genkey() -> anyhow::Result<Vec<u8>> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        // base64
        let key = BASE64_STANDARD.encode(key).as_bytes().to_vec();
        Ok(key)
    }

    fn readkey(path: &Path) -> anyhow::Result<Vec<u8>> {
        let key = fs::read(path)?;
        // base64
        let key = BASE64_STANDARD.decode(key)?;
        Ok(key)
    }

    fn encode(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let key = GenericArray::clone_from_slice(self.key.as_slice());
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, data).unwrap();
        let nonce_vec: &[u8; 12] = &nonce[..12].try_into().unwrap();
        let nonce_vec = [nonce_vec, ciphertext.as_slice()].concat();
        Ok(BASE64_STANDARD.encode(nonce_vec).as_bytes().to_vec())
    }

    fn decode(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let data = BASE64_STANDARD.decode(data)?;
        let key = GenericArray::clone_from_slice(self.key.as_slice());
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = GenericArray::clone_from_slice(&data[0..12]);
        let plaintext = cipher.decrypt(&nonce, &data[12..]).unwrap();
        Ok(plaintext)
    }
}
