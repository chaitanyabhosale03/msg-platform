use sodiumoxide::crypto::box_::{self, PublicKey as SodiumPubKey, SecretKey as SodiumSecKey};
use sodiumoxide::crypto::sign::{self, PublicKey as SignPubKey, SecretKey as SignSecKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicKey {
    pub pk_box: Vec<u8>,
    pub pk_sign: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateKey {
    pub sk_box: Vec<u8>,
    pub sk_sign: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct KeyPair {
    pub public: PublicKey,
    pub private: PrivateKey,
}

impl KeyPair {
    /// Generate new X25519 + Ed25519 keypair
    pub fn generate() -> Self {
        sodiumoxide::init().unwrap();

        let (pk_box, sk_box) = box_::gen_keypair();
        let (pk_sign, sk_sign) = sign::gen_keypair();

        Self {
            public: PublicKey {
                pk_box: pk_box.0.to_vec(),
                pk_sign: pk_sign.0.to_vec(),
            },
            private: PrivateKey {
                sk_box: sk_box.0.to_vec(),
                sk_sign: sk_sign.0.to_vec(),
            },
        }
    }

    /// Load keypair from bytes
    pub fn from_bytes(pub_box: &[u8], sec_box: &[u8], pub_sign: &[u8], sec_sign: &[u8]) -> Self {
        Self {
            public: PublicKey {
                pk_box: pub_box.to_vec(),
                pk_sign: pub_sign.to_vec(),
            },
            private: PrivateKey {
                sk_box: sec_box.to_vec(),
                sk_sign: sec_sign.to_vec(),
            },
        }
    }

    /// Encrypt to recipient public key
    pub fn encrypt_to(&self, recipient_pk: &PublicKey, plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
        sodiumoxide::init().ok();

        let pk = SodiumPubKey::from_slice(&recipient_pk.pk_box)
            .ok_or_else(|| anyhow::anyhow!("Invalid public key"))?;
        let sk = SodiumSecKey::from_slice(&self.private.sk_box)
            .ok_or_else(|| anyhow::anyhow!("Invalid secret key"))?;

        let ciphertext = box_::seal(plaintext, &pk, &sk);
        Ok(ciphertext)
    }

    /// Decrypt from sender public key
    pub fn decrypt_from(&self, sender_pk: &PublicKey, ciphertext: &[u8]) -> anyhow::Result<Vec<u8>> {
        sodiumoxide::init().ok();

        let pk = SodiumPubKey::from_slice(&sender_pk.pk_box)
            .ok_or_else(|| anyhow::anyhow!("Invalid public key"))?;
        let sk = SodiumSecKey::from_slice(&self.private.sk_box)
            .ok_or_else(|| anyhow::anyhow!("Invalid secret key"))?;

        let plaintext = box_::open(ciphertext, &pk, &sk)
            .map_err(|_| anyhow::anyhow!("Decryption failed"))?;
        Ok(plaintext)
    }

    /// Sign message with private key
    pub fn sign(&self, message: &[u8]) -> anyhow::Result<Vec<u8>> {
        sodiumoxide::init().ok();

        let sk = SignSecKey::from_slice(&self.private.sk_sign)
            .ok_or_else(|| anyhow::anyhow!("Invalid signing key"))?;

        Ok(sign::sign(message, &sk).0.to_vec())
    }

    /// Verify signature
    pub fn verify(sender_pk: &PublicKey, signed_message: &[u8]) -> anyhow::Result<Vec<u8>> {
        sodiumoxide::init().ok();

        let pk = SignPubKey::from_slice(&sender_pk.pk_sign)
            .ok_or_else(|| anyhow::anyhow!("Invalid public key"))?;

        let plaintext = sign::verify(signed_message, &pk)
            .map_err(|_| anyhow::anyhow!("Signature verification failed"))?;
        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let kp = KeyPair::generate();
        assert_eq!(kp.public.pk_box.len(), 32);
        assert_eq!(kp.public.pk_sign.len(), 32);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let kp1 = KeyPair::generate();
        let kp2 = KeyPair::generate();

        let plaintext = b"Hello, World!";
        let ciphertext = kp1.encrypt_to(&kp2.public, plaintext).unwrap();
        let decrypted = kp2.decrypt_from(&kp1.public, &ciphertext).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_sign_verify() {
        let kp = KeyPair::generate();
        let message = b"Important message";
        let signed = kp.sign(message).unwrap();
        let verified = KeyPair::verify(&kp.public, &signed).unwrap();

        assert_eq!(message, &verified[..]);
    }
}
