use openssl::symm::{Cipher, Crypter, Mode};

pub trait AESCrypter {
    type ReturnType;
    fn aes_encrypt_bytes(&self, encrypted: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType;
    fn aes_decrypt_bytes(&self, plaintext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType;
    fn aes_compare_slice(&self, encrypted: &[u8], key: &[u8], iv: Option<&[u8]>, plaintext: &[u8]) -> bool;
    /// Compares a wide string with the encrypted data. The implementation of this on the DefaultAESCrypter assumes the
    /// encrypted string is UTF-8, so it writes the utf-8 version of each character of the 'other' string to the buffer
    /// for the encryption and comparison.
    ///
    /// # Arguments
    ///
    /// * `encrypted`: `&[u8]` - Encrypted bytes.
    /// * `key`: `&[u8]` - AES Key for encrypted bytes.
    /// * `iv`: `Option<&[u8]>` - Optional IV for encrypted bytes.
    /// * `plaintext`: `&[u16]` - The plaintext wide string we are comparing against.
    ///
    /// returns: bool
    fn aes_compare_w_str(&self, encrypted: &[u8], key: &[u8], iv: Option<&[u8]>, plaintext: &[u16])
        -> bool;
}

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

#[cfg(feature = "openssl")]
const TEMP_BUFFER_SIZE: usize = 0x100;

#[cfg(feature = "openssl")]
impl AESCrypter for DefaultAesCrypter {
    type ReturnType = std::io::Result<Vec<u8>>;
    fn aes_encrypt_bytes(
        &self,
        plaintext: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Self::ReturnType {
        let mut crypter = Crypter::new(self.cipher, Mode::Encrypt, key, iv)?;
        crypter.pad(true);
        let mut out = vec![0; plaintext.len() + self.cipher.block_size()];
        let count = crypter.update(plaintext, &mut out)?;
        let rest = crypter.finalize(&mut out[count..])?;
        out.truncate(count + rest);
        Ok(out)
    }
    fn aes_decrypt_bytes(
        &self,
        encrypted: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Self::ReturnType {
        let mut crypter = Crypter::new(self.cipher, Mode::Decrypt, key, iv)?;
        crypter.pad(true);
        let mut out = vec![0; encrypted.len() + self.cipher.block_size() * 2];
        let count = crypter.update(encrypted, &mut out)?;
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

        let mut crypter = Crypter::new(self.cipher, Mode::Encrypt, key, iv).unwrap(); // .expect("Could not get Crypter from openssl.")

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
