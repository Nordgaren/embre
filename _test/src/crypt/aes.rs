#[cfg(test)]
mod tests {
    use embre_crypt::aes::{AESCrypter, AESStrCompare, DefaultAesCrypter};

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
