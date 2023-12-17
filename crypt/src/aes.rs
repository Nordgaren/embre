use openssl::symm::*;

pub trait AESCrypter {
    type ReturnType;
    fn aes_encrypt_bytes(self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType;
    fn aes_decrypt_bytes(self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType;
}
pub struct DefaultAesCrypter;
impl AESCrypter for DefaultAesCrypter {
    type ReturnType = std::io::Result<Vec<u8>>;
    fn aes_encrypt_bytes(self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType {
        aes_encrypt_bytes(bytes, key, iv)
    }
    fn aes_decrypt_bytes(self, bytes: &[u8], key: &[u8], iv: Option<&[u8]>) -> Self::ReturnType {
        aes_decrypt_bytes(bytes, key, iv)
    }
}

pub fn aes_decrypt_bytes(file: &[u8], key: &[u8], iv: Option<&[u8]>) -> std::io::Result<Vec<u8>> {
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, iv)?;
    crypter.pad(true);
    let mut out = vec![0; file.len() + cipher.block_size()];
    let count = crypter.update(file, &mut out)?;
    let rest = crypter.finalize(&mut out[count..])?;
    out.truncate(count + rest);
    Ok(out)
}

pub fn aes_encrypt_bytes(file: &[u8], key: &[u8], iv: Option<&[u8]>) -> std::io::Result<Vec<u8>> {
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, iv)?;
    crypter.pad(true);
    let mut out = vec![0; file.len() + cipher.block_size()];
    let count = crypter.update(file, &mut out)?;
    let rest = crypter.finalize(&mut out[count..])?;
    out.truncate(count + rest);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::aes::{aes_decrypt_bytes, aes_encrypt_bytes};

    #[test]
    fn aes_decrypt() {
        let data = [
            0xC4, 0x28, 0x41, 0x56, 0xA5, 0x00, 0xD0, 0xE3, 0x8E, 0x28, 0x85, 0x37, 0x07, 0xBF,
            0xDE, 0xF9,
        ];
        let key: Vec<u8> = (1..=32).collect();
        let bytes = aes_decrypt_bytes(&data, &key[..], Some(&(1..=16).collect::<Vec<u8>>()[..]))
            .expect("Could not decrypt test string");
        assert_eq!(&bytes[..], b"aes_test_string");
    }

    #[test]
    fn aes_encrypt() {
        let data = b"aes_test_string";
        let key: Vec<u8> = (1..=32).collect();
        let bytes = aes_encrypt_bytes(
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
