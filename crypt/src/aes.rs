use openssl::symm::*;

pub trait AESCrypter {
    type ReturnType;
    fn aes_encrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType;
    fn aes_decrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType;
    fn get_cipher(&self) -> Cipher;
}
pub struct DefaultAesCrypter {
    cipher: Cipher,
}
impl Default for DefaultAesCrypter {
    fn default() -> Self {
        Self::new(Cipher::aes_256_cbc())
    }
}
impl DefaultAesCrypter {
    pub fn new(cipher: Cipher) -> Self {
        DefaultAesCrypter {
            cipher
        }
    }
}
impl AESCrypter for DefaultAesCrypter {
    type ReturnType = std::io::Result<Vec<u8>>;
    fn aes_encrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType {
        aes_encrypt_bytes(self.cipher, bytes, key, iv)
    }
    fn aes_decrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType {
        aes_decrypt_bytes(self.cipher, bytes, key, iv)
    }
    fn get_cipher(&self) -> Cipher {
        self.cipher
    }
}

pub fn aes_decrypt_bytes(cipher: Cipher, file: &[u8], key: &[u8], iv: Option<&[u8]>) -> std::io::Result<Vec<u8>> {
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, iv)?;
    crypter.pad(true);
    let mut out = vec![0; file.len() + cipher.block_size()];
    let count = crypter.update(file, &mut out)?;
    let rest = crypter.finalize(&mut out[count..])?;
    out.truncate(count + rest);
    Ok(out)
}

pub fn aes_encrypt_bytes(cipher: Cipher, file: &[u8], key: &[u8], iv: Option<&[u8]>) -> std::io::Result<Vec<u8>> {
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, iv)?;
    crypter.pad(true);
    let mut out = vec![0; file.len() + cipher.block_size()];
    let count = crypter.update(file, &mut out)?;
    let rest = crypter.finalize(&mut out[count..])?;
    out.truncate(count + rest);
    Ok(out)
}

pub fn aes_u8_cmp(cipher: Cipher, buffer: &[u8], key: &[u8], iv: Option<&[u8]>, other: &[u8]) -> bool {
    let block_size = cipher.block_size();
    let size = other.len();
    let len = size+ block_size - (size % block_size);

    if buffer.len() != len {
        return false;
    }

    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, iv).expect("Could not get Crypter from openssl.");

    let mut temp = [0;0x100];
    let mut total = 0;
    for chunk in other.chunks(block_size) {
        let out = &mut temp[..block_size * 2];
        let en = crypter.update(chunk, out).expect("Could not encrypt chunk");
        if en == 0 {
            crypter.finalize(out).expect("Could not encrypt chunk");
        }
        if out[..block_size] != buffer[total..total + block_size] {
            return false
        }
        total += block_size;
        out.fill(0);
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::aes::{AESCrypter, DefaultAesCrypter};

    #[test]
    fn aes_decrypt() {
        let data = [
            0xC4, 0x28, 0x41, 0x56, 0xA5, 0x00, 0xD0, 0xE3, 0x8E, 0x28, 0x85, 0x37, 0x07, 0xBF,
            0xDE, 0xF9,
        ];
        let key: Vec<u8> = (1..=32).collect();
        let crypter = DefaultAesCrypter::default();
        let bytes = crypter.aes_decrypt_bytes(&data, &key[..], Some(&(1..=16).collect::<Vec<u8>>()[..]))
            .expect("Could not decrypt test string");
        assert_eq!(&bytes[..], b"aes_test_string");
    }

    #[test]
    fn aes_encrypt() {
        let data = b"aes_test_string";
        let key: Vec<u8> = (1..=32).collect();
        let crypter = DefaultAesCrypter::default();
        let bytes = crypter.aes_encrypt_bytes(
            &data[..],
            &key[..],
            Some(&(1..=16).collect::<Vec<u8>>()[..]),
        )
        .expect("Could not encrypt test string");
        assert_eq!(
            &bytes[..],
            [
                0xC4, 0x28, 0x41, 0x56, 0xA5, 0x00, 0xD0, 0xE3, 0x8E, 0x28, 0x85, 0x37, 0x07, 0xBF,
                0xDE, 0xF9
            ]
        );
    }
}
