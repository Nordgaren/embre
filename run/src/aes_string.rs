use std::marker::PhantomData;
use embre_crypt::aes::{AESCrypter, DefaultAesCrypter};
use crate::StringResource;

pub struct AESResource<'a, T, C: AESCrypter = DefaultAesCrypter> {
    memory: &'a u8,
    crypter: C,
    phantom: PhantomData<T>,
}
impl<'a, T> AESResource<'a, T> {
    pub fn new() -> AESResource<'a, T> {
        Self::new_in(DefaultAesCrypter)
    }
}

impl<'a, T, C: AESCrypter> AESResource<'a, T, C> {
    pub fn new_in(crypter: C) -> AESResource<'a, T, C> {
        AESResource {
            memory: &0,
            crypter,
            phantom: PhantomData,
        }
    }
}
