#![allow(unused)]
use openssl::encrypt::{Decrypter, Encrypter};
use openssl::pkey::PKey;
use openssl::rsa::{Padding, Rsa};

pub(crate) fn rsa_decrypt_bytes(file: &[u8], key: &[u8]) -> std::io::Result<Vec<u8>> {
    let public_key = Rsa::private_key_from_pem(key)?;
    let p_key = PKey::from_rsa(public_key)?;
    let mut decrypter = Decrypter::new(&p_key)?;
    decrypter.set_rsa_padding(Padding::PKCS1)?;
    let buffer_len = decrypter.decrypt_len(file)?;
    let mut buffer = vec![0; buffer_len];
    let len = decrypter.decrypt(file, &mut buffer[..])?;
    buffer.truncate(len);
    Ok(buffer)
}

pub(crate) fn rsa_encrypt_bytes(file: &[u8], key: &[u8]) -> std::io::Result<Vec<u8>> {
    let public_key = Rsa::public_key_from_pem_pkcs1(key)?;
    let p_key = PKey::from_rsa(public_key)?;
    let mut encrypter = Encrypter::new(&p_key)?;
    encrypter.set_rsa_padding(Padding::PKCS1)?;
    let buffer_len = encrypter.encrypt_len(file)?;
    let mut buffer = vec![0; buffer_len];
    encrypter.encrypt(file, &mut buffer[..])?;
    Ok(buffer)
}
#[cfg(test)]
mod tests {
    use std::fs;
    use crate::rsa::{rsa_decrypt_bytes, rsa_encrypt_bytes};

    #[test]
    fn rsa_decrypt() {
        let data = [0x97, 0xF7, 0x47, 0xFA, 0xF4, 0x49, 0xD4, 0x77, 0x1A, 0x6, 0xD9, 0x5C, 0x88, 0xC3, 0x57, 0x6C, 0xF3, 0x1F, 0x4F, 0x9, 0x78, 0x21, 0x78, 0xEB, 0xFA, 0xE0, 0x46, 0x91, 0x92, 0x12, 0x3E, 0xEF, 0xD3, 0x4E, 0xFB, 0x56, 0xA4, 0x40, 0xDA, 0xC, 0x47, 0xFC, 0x2A, 0x15, 0x2F, 0x7, 0x30, 0xD, 0x7A, 0x64, 0xEA, 0xF9, 0x5E, 0xFC, 0x5C, 0xE3, 0x46, 0x41, 0xB5, 0x87, 0xA0, 0xAB, 0x7E, 0xBF, 0x2C, 0x7F, 0x6C, 0x2F, 0xBB, 0x90, 0x78, 0x4B, 0xB6, 0x40, 0xD2, 0x0, 0x35, 0xB9, 0xCA, 0xD5, 0x43, 0xE9, 0xA0, 0xA0, 0xB1, 0x52, 0x95, 0x9A, 0x46, 0xB2, 0x2D, 0x1A, 0x40, 0x90, 0x3A, 0x7C, 0xDF, 0xDA, 0xE8, 0x6, 0x2A, 0xA6, 0x1A, 0x6C, 0x22, 0x73, 0xEC, 0xB8, 0xBF, 0xC, 0xE7, 0x6C, 0x6, 0xAF, 0xE2, 0xF6, 0xCE, 0xE8, 0xBB, 0x84, 0x26, 0x1A, 0xE9, 0x1A, 0xBB, 0x0, 0xC2, 0x5A, 0x49, 0x4, 0xE7, 0x8A, 0x81, 0x7B, 0xE5, 0xAE, 0xC4, 0xCF, 0x6, 0xD8, 0xF2, 0x46, 0x4C, 0xBE, 0x36, 0xC1, 0xB2, 0x52, 0xDB, 0xF7, 0x1, 0x2, 0xE0, 0x92, 0x13, 0x69, 0x7B, 0x1A, 0xF1, 0x1B, 0x8F, 0xA8, 0x63, 0xA0, 0xC4, 0xF5, 0x67, 0x27, 0xB3, 0x4C, 0xB0, 0xC7, 0x43, 0x7F, 0x0, 0x20, 0x95, 0x1, 0x24, 0x17, 0x71, 0x86, 0xDA, 0x7F, 0x20, 0x7E, 0x97, 0x40, 0xF3, 0x3D, 0xC3, 0x3D, 0x16, 0xA5, 0x3B, 0x5C, 0x5C, 0x43, 0x5C, 0x9, 0x79, 0x6A, 0xD1, 0xD1, 0xC5, 0x85, 0x70, 0x31, 0xFA, 0xCA, 0xC1, 0x99, 0x4D, 0xE, 0x88, 0x86, 0xD3, 0x83, 0x87, 0x83, 0xDC, 0x48, 0xD6, 0x6B, 0xDA, 0x9B, 0x84, 0xB6, 0x60, 0x7, 0x7E, 0x2, 0xE1, 0x9E, 0x3, 0x1D, 0x9E, 0x12, 0xEF, 0x3B, 0x1B, 0xF, 0x9F, 0x8D, 0x66, 0xF6, 0x46, 0x1D, 0xDA, 0xB9, 0x54, 0x64, 0x1F, 0x5F, 0x9E, 0xE7];
        let key = fs::read("test_key").expect("Could not read 'test_key'. Make sure it is in the crypt directory.");
        let bytes = rsa_decrypt_bytes(&data, &key[..]).expect("Could not decrypt test string");
        assert_eq!(&bytes[..], b"rsa_test_string");
    }

    #[test]
    fn rsa_encrypt() {
        let data = b"rsa_test_string";
        let key = fs::read("test_key.pub").expect("Could not read 'test_key.pub'. Make sure it is in the crypt directory.");
        let bytes = rsa_encrypt_bytes(&data[..], &key[..]).expect("Could not encrypt test string");
        let key = fs::read("test_key").expect("Could not read test private key");
        let unenc = rsa_decrypt_bytes(&bytes[..], &key[..]).expect("Could not encrypt test string");
        assert_eq!(&unenc[..], b"rsa_test_string");
    }

}

pub(crate) fn rsa_encrypt_bytes_old(file: &[u8], key: &[u8]) -> std::io::Result<Vec<u8>> {

    // Read the public key from a PEM file
    let public_key = Rsa::public_key_from_pem(key)?;

    // Encrypt the data using the public key
    let key_size = public_key.size() as usize;
    let mut plaintext = file.to_vec();
    let pad = key_size - (plaintext.len() % key_size);
    plaintext.resize(plaintext.len() + pad, 0);

    let mut len = 0;
    let mut encrypted = vec![];

    while len < plaintext.len() {
        let mut buffer = vec![0; key_size];
        let next_block = plaintext.len() % (len + key_size);
        let block_data = &plaintext[len..next_block];
        len += public_key.public_encrypt(block_data, &mut buffer, Padding::PKCS1)?;
        encrypted.extend_from_slice(&buffer[..]);
    }

    // Trim the encrypted data to the actual length
    encrypted.truncate(len);

    Ok(encrypted)
}

pub(crate) fn rsa_decrypt_bytes_old(file: &[u8], key: &[u8]) -> std::io::Result<Vec<u8>> {

    // Read the private key from a PEM file
    let private_key = Rsa::private_key_from_pem(key)?;

    // Decrypt the data using the private key
    let key_size = private_key.size() as usize;

    let mut len = 0;
    let mut decrypted_data = vec![];

    while len < file.len() {
        let mut buffer = vec![0; key_size];
        let next_block = len + key_size;
        let block_data = &file[len..next_block];
        let dec = private_key.private_decrypt(block_data, &mut buffer, Padding::PKCS1)?;
        len += dec;
        decrypted_data.extend_from_slice(&buffer[..]);
        if dec < key_size {
            break;
        }
    }

    // Trim the decrypted data to the actual length
    decrypted_data.truncate(len);

    return Ok(decrypted_data);
}
