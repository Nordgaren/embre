use openssl::symm::*;

pub trait AESCrypter {
    type ReturnType;
    fn aes_encrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType;
    fn aes_decrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType;
    fn aes_compare_slice(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>, other: &[u8])
                         -> bool;
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
        DefaultAesCrypter { cipher }
    }
    pub fn get_cipher(&self) -> Cipher {
        self.cipher
    }
}
impl AESCrypter for DefaultAesCrypter {
    type ReturnType = std::io::Result<Vec<u8>>;
    fn aes_encrypt_bytes(&self, encrypted: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType {
        let mut crypter = Crypter::new(self.cipher, Mode::Encrypt, key, iv)?;
        crypter.pad(true);
        let mut out = vec![0; encrypted.len() + self.cipher.block_size()];
        let count = crypter.update(encrypted, &mut out)?;
        let rest = crypter.finalize(&mut out[count..])?;
        out.truncate(count + rest);
        Ok(out)
    }
    fn aes_decrypt_bytes(&self, plaintext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType {
        let mut crypter = Crypter::new(self.cipher, Mode::Decrypt, key, iv)?;
        crypter.pad(true);
        let mut out = vec![0; plaintext.len() + self.cipher.block_size()];
        let count = crypter.update(plaintext, &mut out)?;
        let rest = crypter.finalize(&mut out[count..])?;
        out.truncate(count + rest);
        Ok(out)
    }
    fn aes_compare_slice(
        &self,
        encrypted: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
        plaintext: &[u8],
    ) -> bool {
        let block_size = self.cipher.block_size();
        let size = plaintext.len();
        let len = size + block_size - (size % block_size);

        if encrypted.len() != len {
            return false;
        }

        let mut crypter = Crypter::new(self.cipher, Mode::Encrypt, key, iv)
            .expect("Could not get Crypter from openssl.");

        let mut temp = [0; 0x100];
        let mut total = 0;
        for chunk in plaintext.chunks(block_size) {
            let out = &mut temp[..block_size * 2];
            let written = crypter.update(chunk, out).expect("Could not encrypt chunk");
            if written == 0 {
                crypter.finalize(out).expect("Could not encrypt chunk");
            }
            if out[..block_size] != encrypted[total..total + block_size] {
                return false;
            }
            total += block_size;
            out.fill(0);
        }

        true
    }
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
        let bytes = crypter
            .aes_decrypt_bytes(&data, &key[..], Some(&(1..=16).collect::<Vec<u8>>()[..]))
            .expect("Could not decrypt test string");
        assert_eq!(&bytes[..], b"aes_test_string");
    }

    #[test]
    fn aes_encrypt() {
        let data = b"aes_test_string";
        let key: Vec<u8> = (1..=32).collect();
        let crypter = DefaultAesCrypter::default();
        let bytes = crypter
            .aes_encrypt_bytes(
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
    #[test]
    fn aes_string_compare() {
        let data = b"aes_test_string";
        let key: Vec<u8> = (1..=32).collect();
        let iv = (1..=16).collect::<Vec<u8>>();
        let crypter = DefaultAesCrypter::default();
        let bytes = crypter
            .aes_encrypt_bytes(&data[..], &key[..], Some(&iv[..]))
            .expect("Could not encrypt test string");

        assert!(crypter.aes_compare_slice(&bytes[..], &key[..], Some(&iv[..]), data));
    }
}
