use aes::Aes256;
use base64::engine::general_purpose;
use base64::{alphabet, engine, Engine};
use ctr::cipher::{KeyIvInit, StreamCipher};
use ctr::Ctr128BE;
use p384::ecdh::diffie_hellman;
use p384::ecdsa::SigningKey;
use p384::pkcs8::DecodePublicKey;
use p384::PublicKey;
use sha2::{Digest, Sha256};
use std::error::Error;
use binary_utils::binary::Writer;

type Aes256Ctr = Ctr128BE<Aes256>;

pub struct Encryption {
    key: Vec<u8>,
    decrypt_cipher: Aes256Ctr,
    decrypt_counter: u64,
    encrypt_cipher: Aes256Ctr,
    encrypt_counter: u64,
}

impl Encryption {
    pub fn new(encryption_key: &[u8], iv: &[u8]) -> Result<Self, Box<dyn Error>> {
        let decrypt_cipher = Aes256Ctr::new_from_slices(encryption_key, iv).expect("Decrypt Cipher Creating Error");
        let encrypt_cipher = Aes256Ctr::new_from_slices(encryption_key, iv).expect("Encrypt Cipher Creating Error");

        Ok(Encryption {
            key: encryption_key.to_vec(),
            decrypt_cipher,
            decrypt_counter: 0,
            encrypt_cipher,
            encrypt_counter: 0,
        })
    }

    pub fn fake_gcm(encryption_key: &[u8]) -> Result<Self, Box<dyn Error>> {
        let mut iv = [0u8; 16];
        iv[..12].copy_from_slice(&encryption_key[..12]);
        iv[12..].copy_from_slice(&[0x00, 0x00, 0x00, 0x02]);
        Self::new(encryption_key, &iv)
    }

    pub fn cfb8(encryption_key: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::new(encryption_key, &encryption_key[..16])
    }

    pub fn decrypt<'b>(&mut self, encrypted: &'b mut [u8]) -> Result<&'b [u8], Box<dyn Error>> {
        if encrypted.len() < 9 { return Err("Payload is too short".into()); }

        self.decrypt_cipher.apply_keystream(encrypted);

        let payload_len = encrypted.len() - 8;
        let (payload, expected_checksum) = encrypted.split_at_mut(payload_len);

        let packet_counter = self.decrypt_counter;
        self.decrypt_counter += 1;

        let actual_checksum = self.calculate_checksum(packet_counter, &payload)?;
        if actual_checksum != expected_checksum {
            return Err(format!("Invalid checksum on packet {}", packet_counter).into());
        }
        Ok(payload)
    }

    /*pub fn encrypt(&mut self, payload: &[u8], stream: &mut Writer) -> Result<(), Box<dyn Error>> {
        let packet_counter = self.encrypt_counter;
        self.encrypt_counter += 1;

        let checksum = self.calculate_checksum(packet_counter, payload)?;

        stream.put(payload);
        stream.put(&checksum);

        self.encrypt_cipher.apply_keystream(stream.as_mut_slice());

        Ok(())
    }*/

    pub fn encrypt_in_place(&mut self, stream: &mut Writer, from: usize) -> Result<(), Box<dyn Error>> {
        let counter = self.encrypt_counter;
        self.encrypt_counter += 1;

        let checksum = self.calculate_checksum(counter, &stream.as_slice()[from..])?;
        stream.put(&checksum);

        self.encrypt_cipher.apply_keystream(&mut stream.as_mut_slice()[from..]);
        Ok(())
    }

    #[inline]
    fn calculate_checksum(&self, counter: u64, payload: &[u8]) -> Result<[u8; 8], Box<dyn Error>> {
        let mut hasher = Sha256::new();
        hasher.update(&counter.to_le_bytes());
        hasher.update(payload);
        hasher.update(&self.key);

        let hash = hasher.finalize();
        let mut checksum = [0u8; 8];
        checksum.copy_from_slice(&hash[..8]);

        Ok(checksum)
    }

    pub fn b64_url_decode(base64_url: &str) -> Result<String, Box<dyn Error>> {
        const BASE64_URL: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
        let b64_url = BASE64_URL.decode(base64_url)?;
        Ok(String::from_utf8(b64_url)?)
    }

    pub fn b64_url_encode(input: &[u8]) -> String {
        const BASE64_URL: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
        BASE64_URL.encode(input)
    }
}

#[inline]
pub fn generate_key(secret: &[u8], salt: Vec<u8>) -> Vec<u8> {
    let hex_secret = hex::encode(secret);
    let hex_secret = if hex_secret.len() < 96 { format!("{:0>96}", hex_secret) } else { hex_secret };
    let secret_bytes = hex::decode(hex_secret).unwrap();
    let combined = [salt, secret_bytes].concat();
    let mut hasher = Sha256::new();
    hasher.update(&combined);
    hasher.finalize().to_vec()
}

#[inline]
pub fn generate_shared_secret(local_private: &SigningKey, remote_public: &PublicKey) -> Vec<u8> {
    let shared = diffie_hellman(local_private.as_nonzero_scalar(), remote_public.as_affine());
    shared.raw_secret_bytes().to_vec()
}

#[inline]
pub fn parse_der_public_key(der_key: &[u8]) -> PublicKey {
    PublicKey::from_public_key_der(der_key).expect("DER To Public Key Convert Error")
}

#[inline]
pub fn fix_base64_padding(s: &str) -> String {
    let rem = s.len() % 4;
    if rem == 0 { s.to_string() } else {
        let pad = 4 - rem;
        let mut s = s.to_string();
        s.extend(std::iter::repeat('=').take(pad));
        s
    }
}