pub trait AESCrypter {
    type ReturnType;
    fn aes_encrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType;
    fn aes_decrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType;
    fn aes_compare_slice(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>, other: &[u8]) -> bool;
    fn aes_compare_w_str(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>, other: &[u16])
        -> bool;
}

#[cfg(feature = "openssl")]
pub mod openssl {
    use crate::aes::AESCrypter;
    use openssl::symm::*;

    pub struct DefaultAesCrypter {
        #[cfg(feature = "openssl")]
        cipher: Cipher,
    }

impl Default for DefaultAesCrypter {
    fn default() -> Self {
        if cfg!(feature = "openssl") {
            Self::new(Cipher::aes_256_cbc())
        } else {
            unimplemented!("DefaultAesCrypter")
        }
    }
}
#[cfg(feature = "openssl")]
impl DefaultAesCrypter {
    pub fn new(cipher: Cipher) -> Self {
        DefaultAesCrypter { cipher }
    }
    pub fn get_cipher(&self) -> Cipher {
        self.cipher
    }
}
    #[cfg(not(feature = "openssl"))]
    impl AESCrypter for DefaultAesCrypter {
        type ReturnType = ();

        fn aes_encrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType {
            unimplemented!("AESCrypter for DefaultAesCrypter")
        }

        fn aes_decrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType {
            unimplemented!("AESCrypter for DefaultAesCrypter")
        }

        fn aes_compare_slice(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>, other: &[u8]) -> bool {
            unimplemented!("AESCrypter for DefaultAesCrypter")
        }
    }
#[cfg(not(feature = "openssl"))]
impl AESCrypter for DefaultAesCrypter {
    type ReturnType = ();

    fn aes_encrypt_bytes(&self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType {
        unimplemented!("AESCrypter for DefaultAesCrypter")
    }

    const TEMP_BUFFER_SIZE: usize = 0x100;

    impl AESCrypter for DefaultAesCrypter {
        type ReturnType = std::io::Result<Vec<u8>>;
        fn aes_encrypt_bytes(
            &self,
            encrypted: &[u8],
            key: &[u8],
            iv: Option<&[u8]>,
        ) -> Self::ReturnType {
            let mut crypter = Crypter::new(self.cipher, Mode::Encrypt, key, iv)?;
            crypter.pad(true);
            let mut out = vec![0; encrypted.len() + self.cipher.block_size()];
            let count = crypter.update(encrypted, &mut out)?;
            let rest = crypter.finalize(&mut out[count..])?;
            out.truncate(count + rest);
            Ok(out)
        }
        fn aes_decrypt_bytes(
            &self,
            plaintext: &[u8],
            key: &[u8],
            iv: Option<&[u8]>,
        ) -> Self::ReturnType {
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
            .unwrap(); // .expect("Could not get Crypter from openssl.")

            let mut temp = [0; TEMP_BUFFER_SIZE];
            let mut total = 0;
            for chunk in plaintext.chunks(block_size) {
                let out = &mut temp[..block_size * 2];
                // "Could not encrypt chunk"
                let written = crypter.update(chunk, out).unwrap();
                if written == 0 {
                    // "Could not encrypt chunk"
                    crypter.finalize(out).unwrap();
                }
                if out[..block_size] != encrypted[total..total + block_size] {
                    return false;
                }
                total += block_size;
                out.fill(0);
            }

            true
        }
        fn aes_compare_w_str(
            &self,
            encrypted: &[u8],
            key: &[u8],
            iv: Option<&[u8]>,
            plaintext: &[u16],
        ) -> bool {
            fn copy_u16_to_u8(dest: &mut [u8], src: &[u16]) {
                for i in 0..src.len() {
                    dest[i] = src[i] as u8;
                }
            }

            let block_size = self.cipher.block_size();
            let size = plaintext.len();
            let len = size + block_size - (size % block_size);

            if encrypted.len() != len {
                return false;
            }

            let mut crypter = Crypter::new(self.cipher, Mode::Encrypt, key, iv)
                // "Could not get Crypter from openssl."
                .unwrap();

            let mut temp = [0; TEMP_BUFFER_SIZE];
            let mut temp_s = [0; TEMP_BUFFER_SIZE];
            let mut total = 0;
            for chunk in plaintext.chunks(block_size) {
                let chunk_u8 = &mut temp_s[..chunk.len()];
                copy_u16_to_u8(chunk_u8, chunk);

                let out = &mut temp[..block_size * 2];
                out.fill(0);
                // "Could not encrypt chunk"
                let written = crypter.update(chunk_u8, out).unwrap();
                if written == 0 {
                    // "Could not encrypt chunk"
                    crypter.finalize(out).unwrap();
                }
                if out[..block_size] != encrypted[total..total + block_size] {
                    return false;
                }
                total += block_size;
                chunk_u8.fill(0);
            }

            true
        }
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

    #[test]
    fn aes_string_compare_long() {
        let data = b"aes_test_string that is longer than a block";
        let key: Vec<u8> = (1..=32).collect();
        let iv = (1..=16).collect::<Vec<u8>>();
        let crypter = DefaultAesCrypter::default();
        let bytes = crypter
            .aes_encrypt_bytes(&data[..], &key[..], Some(&iv[..]))
            .expect("Could not encrypt test string");

        assert!(crypter.aes_compare_slice(&bytes[..], &key[..], Some(&iv[..]), data));
    }

    #[test]
    fn aes_w_string_compare() {
        let data = b"aes_test_string";
        let key: Vec<u8> = (1..=32).collect();
        let iv = (1..=16).collect::<Vec<u8>>();
        let crypter = DefaultAesCrypter::default();
        let bytes = crypter
            .aes_encrypt_bytes(&data[..], &key[..], Some(&iv[..]))
            .expect("Could not decrypt test string");

        let w_string: Vec<u16> = data.iter().map(|b| *b as u16).collect();

        let crypter = DefaultAesCrypter::default();
        assert!(crypter.aes_compare_w_str(&bytes, &key[..], Some(&iv[..]), &w_string[..]));
    }

    #[test]
    fn aes_w_string_compare_long() {
        let data = b"aes_test_string that is longer than a block";
        let key: Vec<u8> = (1..=32).collect();
        let iv = (1..=16).collect::<Vec<u8>>();
        let crypter = DefaultAesCrypter::default();
        let bytes = crypter
            .aes_encrypt_bytes(&data[..], &key[..], Some(&iv[..]))
            .expect("Could not decrypt test string");

        let w_string: Vec<u16> = data.iter().map(|b| *b as u16).collect();

        let crypter = DefaultAesCrypter::default();
        assert!(crypter.aes_compare_w_str(&bytes, &key[..], Some(&iv[..]), &w_string[..]));
    }
}
