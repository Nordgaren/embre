use openssl::symm::{Cipher, Crypter, Mode};

/// `AESCrypter` trait allows the implementer to specify the input and return type. The implementation of this for
/// `DefaultAESCrypter` has an InType of `&[u8]` and a ReturnType of`Result<Vec<u8>>`.   
///
/// You could implement an `AESCrypter` that takes in a `&mut [u8]`, and passes back a bool on whether or the function
/// succeeded in encrypting the data.
pub trait AESCrypter<'a> {
    /// The input type for the encrypted or decrypted data, for the encrypt and decrypt functions.  
    /// `DefaultAESCrypter` accepts a `&[u8]`.  
    type InType;
    /// The return type for the encrypt and decrypt functions.  
    /// `DefaultAESCrypter` returns a `Result<Vec<u8>>`.  
    type ReturnType;
    /// Encrypts the data passed to the function. The implementation of this for `DefaultAESCrypter` accepts a `&[u8]`
    /// and passes back a `Result<Vec<u8>>` with the new encrypted data, and leaves the input untouched.
    ///
    /// # Arguments
    ///
    /// * `plaintext`: `Self::InType` - The data to be encrypted.  
    /// * `key`: `&[u8]` - AES Key for encrypted bytes.
    /// * `iv`: `Option<&[u8]>` - Optional IV for encrypted bytes.
    ///
    /// returns: Self::ReturnType
    fn aes_encrypt_bytes(
        &self,
        plaintext: Self::InType,
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Self::ReturnType;
    /// Decrypts the data passed to the function. The implementation of this for `DefaultAESCrypter` accepts a `&[u8]`
    /// and passes back a Result<Vec<u8>> with the new decrypted data, and leaves the input untouched.   
    ///
    /// # Arguments
    ///
    /// * `encrypted`: `&[u8]` - The data to be decrypted.  
    /// * `key`: `&[u8]` - AES Key for encrypted bytes.
    /// * `iv`: `Option<&[u8]>` - Optional IV for encrypted bytes.
    ///
    /// returns: Self::ReturnType
    fn aes_decrypt_bytes(
        &self,
        encrypted: Self::InType,
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Self::ReturnType;
}

/// Separate trait for string compare functions. The implementation for `DefaultAESCrypter` checks if the result would be
/// the same length, first. Then it uses a local buffer, chunks the string into slices the same size as the block size,
/// and encrypts the slices into the local buffer and compares the part of the encrypted chunk to the newly encrypted chunk
/// of data, until it gets through the whole string. It does the same for the wide string implementation, except it uses
/// an additional buffer to write out the UTF-8 version of the wide string character to an additional buffer for comparison.
pub trait AESStrCompare {
    /// Compares a string with the encrypted data. The implementation of this on the `DefaultAESCrypter` assumes the
    /// encrypted and plaintext string is UTF-8.
    ///
    /// # Arguments
    ///
    /// * `encrypted`: `&[u8]` - Encrypted bytes.
    /// * `key`: `&[u8]` - AES Key for encrypted bytes.
    /// * `iv`: `Option<&[u8]>` - Optional IV for encrypted bytes.
    /// * `plaintext`: `&[u8]` - The plaintext UTF-8 string we are comparing against.
    ///
    /// returns: bool
    fn aes_compare_slice(
        &self,
        encrypted: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
        plaintext: &[u8],
    ) -> bool;
    /// Compares a wide string with the encrypted data. The implementation of this on the `DefaultAESCrypter` assumes the
    /// encrypted string is UTF-8, so it writes the ASCII version of each character of the 'plaintext' wide string to an
    /// additional buffer for the encryption and comparison.
    ///
    /// # Arguments
    ///
    /// * `encrypted`: `&[u8]` - Encrypted bytes.
    /// * `key`: `&[u8]` - AES Key for encrypted bytes.
    /// * `iv`: `Option<&[u8]>` - Optional IV for encrypted bytes.
    /// * `plaintext`: `&[u16]` - The plaintext wide string we are comparing against.
    ///
    /// returns: bool
    fn aes_compare_w_str(
        &self,
        encrypted: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
        plaintext: &[u16],
    ) -> bool;
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
impl<'a> AESCrypter<'a> for DefaultAesCrypter {
    type InType = ();
    type ReturnType = ();

    fn aes_encrypt_bytes(
        &self,
        plaintext: Self::InType,
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Self::ReturnType {
        unimplemented!("AESCrypter for DefaultAesCrypter")
    }
    fn aes_decrypt_bytes(
        &self,
        encrypted: Self::InType,
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Self::ReturnType {
        unimplemented!("AESCrypter for DefaultAesCrypter")
    }
}

#[cfg(not(feature = "openssl"))]
impl AESStrCompare for DefaultAesCrypter {
    fn aes_compare_slice(
        &self,
        encrypted: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
        plaintext: &[u8],
    ) -> bool {
        unimplemented!("AESCrypter for DefaultAesCrypter")
    }
    fn aes_compare_w_str(
        &self,
        encrypted: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
        plaintext: &[u16],
    ) -> bool {
        unimplemented!("AESCrypter for DefaultAesCrypter")
    }
}

#[cfg(feature = "openssl")]
impl<'a> AESCrypter<'a> for DefaultAesCrypter {
    type InType = &'a [u8];
    // Yea, shut up. :P
    type ReturnType = std::io::Result<Vec<u8>>;
    /// Encrypts the data passed to the function. The default implementation of this for `DefaultAESCrypter` accepts a `&[u8]`
    /// and passes back a `Result<Vec<u8>>` with the new encrypted data, and leaves the input untouched. You could implement
    /// an `AESCrypter` that takes in a `&mut [u8]`, and passes back a bool on whether or the function succeeded in encrypting
    /// the data. It is setup this way so that the user has more control over what their implementation of the trait is.  
    ///
    /// # Arguments
    ///
    /// * `plaintext`: `&[u8]` - The data to be encrypted.  
    /// * `key`: `&[u8]` - AES Key for encrypted bytes.
    /// * `iv`: `Option<&[u8]>` - Optional IV for encrypted bytes.
    ///
    /// returns: std::io::Result<Vec<u8>>
    fn aes_encrypt_bytes(
        &self,
        plaintext: Self::InType,
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
    /// Decrypts the data passed to the function. The default implementation of this for `DefaultAESCrypter` accepts a `&[u8]`
    /// and passes back a Result<Vec<u8>> with the new decrypted data, and leaves the input untouched. You could implement
    /// an `AESCrypter` that takes in a `&mut [u8]`, and passes back a bool on whether or the function succeeded in decrypting
    /// the data. It is setup this way so that the user has more control over what their implementation of the trait is.  
    ///
    /// # Arguments
    ///
    /// * `encrypted`: `&[u8]` - The data to be decrypted.  
    /// * `key`: `&[u8]` - AES Key for encrypted bytes.
    /// * `iv`: `Option<&[u8]>` - Optional IV for encrypted bytes.
    ///
    /// returns: std::io::Result<Vec<u8>>
    fn aes_decrypt_bytes(
        &self,
        encrypted: Self::InType,
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
}

#[cfg(feature = "openssl")]
const TEMP_BUFFER_SIZE: usize = 0x100;

#[cfg(feature = "openssl")]
impl AESStrCompare for DefaultAesCrypter {
    /// Compares a string with the encrypted data. The implementation of this on the `DefaultAESCrypter` assumes the
    /// encrypted and plaintext string is UTF-8. checks if the result would be the same length, first. Then it uses a
    /// local buffer, chunks the string into slices the same size as the block size, and encrypts the slices into the local
    /// buffer and compares the part of the encrypted chunk to the newly encrypted chunk of data, until it gets through
    /// the whole string.
    ///
    /// # Arguments
    ///
    /// * `encrypted`: `&[u8]` - Encrypted bytes.
    /// * `key`: `&[u8]` - AES Key for encrypted bytes.
    /// * `iv`: `Option<&[u8]>` - Optional IV for encrypted bytes.
    /// * `plaintext`: `&[u8]` - The plaintext UTF-8 string we are comparing against.
    ///
    /// returns: bool
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
    /// Compares a string with the encrypted data. The implementation of this on the `DefaultAESCrypter` assumes the
    /// encrypted and plaintext string is UTF-8. checks if the result would be the same length, first. Then it uses a
    /// local buffer, chunks the string into slices the same size as the block size, moves the ASCII value of the wide
    /// string character into a secondary buffer, and encrypts the secondary buffer into the local buffer and compares
    /// the part of the encrypted chunk to the newly encrypted chunk of data, until it gets through the whole wide string.
    ///
    /// # Arguments
    ///
    /// * `encrypted`: `&[u8]` - Encrypted bytes.
    /// * `key`: `&[u8]` - AES Key for encrypted bytes.
    /// * `iv`: `Option<&[u8]>` - Optional IV for encrypted bytes.
    /// * `plaintext`: `&[u16]` - The plaintext UTF-8 string we are comparing against.
    ///
    /// returns: bool
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
