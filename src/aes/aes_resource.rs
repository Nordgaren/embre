use embre_crypt::aes::{AESCrypter, DefaultAesCrypter};
use std::marker::PhantomData;
#[derive(Debug)]
pub struct AESResource<'a, T, C: AESCrypter = DefaultAesCrypter> {
    pub(super) resource: &'a [u8],
    pub(super) key: &'a [u8],
    pub(super) iv: Option<&'a [u8]>,
    pub(super) crypter: C,
    pub(super) phantom_data: PhantomData<T>,
}
impl<'a, T> AESResource<'a, T> {
    pub fn new(cipher_text: &'a [u8], key: &'a [u8], iv: Option<&'a [u8]>) -> AESResource<'a, T> {
        Self::new_from(cipher_text, key, iv, DefaultAesCrypter::default())
    }
    pub fn to_plaintext_data(&self) -> std::io::Result<Vec<u8>> {
        self.crypter
            .aes_decrypt_bytes(self.resource, self.key, self.iv)
    }
}
impl<'a, T, C: AESCrypter> AESResource<'a, T, C> {
    pub fn new_from(
        cipher_text: &'a [u8],
        key: &'a [u8],
        iv: Option<&'a [u8]>,
        crypter: C,
    ) -> AESResource<'a, T, C> {
        AESResource {
            resource: cipher_text,
            key,
            iv,
            crypter,
            phantom_data: PhantomData,
        }
    }
}
impl<T> PartialEq<Self> for AESResource<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.resource == other.resource && self.key == other.key && self.iv == other.iv
    }
}
impl<T> Eq for AESResource<'_, T> {}
impl<T> PartialEq<[u8]> for AESResource<'_, T> {
    fn eq(&self, other: &[u8]) -> bool {
        self.crypter
            .aes_compare_string(self.resource, self.key, self.iv, other)
    }
}
impl<T> PartialEq<&[u8]> for AESResource<'_, T> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.eq(*other)
    }
}
impl<T> PartialEq<Vec<u8>> for AESResource<'_, T> {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.eq(&other[..])
    }
}
