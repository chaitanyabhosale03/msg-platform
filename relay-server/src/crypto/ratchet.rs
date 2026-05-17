use sodiumoxide::crypto::aead::chacha20poly1305_ietf as chacha;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RatchetState {
    pub key: Vec<u8>,
    pub counter: u64,
    pub nonce: Vec<u8>,
}

impl RatchetState {
    pub fn new() -> Self {
        sodiumoxide::init().ok();
        let key = sodiumoxide::randombytes::randombytes(32);
        let nonce = sodiumoxide::randombytes::randombytes(12);

        Self {
            key,
            counter: 0,
            nonce,
        }
    }

    /// Encrypt with ChaCha20-Poly1305
    pub fn encrypt(&mut self, plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let key = chacha::Key::from_slice(&self.key)
            .ok_or_else(|| anyhow::anyhow!("Invalid key"))?
        let nonce = chacha::Nonce::from_slice(&self.nonce)
            .ok_or_else(|| anyhow::anyhow!("Invalid nonce"))?

        let ciphertext = chacha::seal(plaintext, None, &nonce, &key);
        self.counter += 1;
        Ok(ciphertext)
    }

    /// Decrypt with ChaCha20-Poly1305
    pub fn decrypt(&mut self, ciphertext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let key = chacha::Key::from_slice(&self.key)
            .ok_or_else(|| anyhow::anyhow!("Invalid key"))?
        let nonce = chacha::Nonce::from_slice(&self.nonce)
            .ok_or_else(|| anyhow::anyhow!("Invalid nonce"))?

        let plaintext = chacha::open(ciphertext, None, &nonce, &key)
            .map_err(|_| anyhow::anyhow!("Decryption failed"))?
        self.counter += 1;
        Ok(plaintext)
    }
}
