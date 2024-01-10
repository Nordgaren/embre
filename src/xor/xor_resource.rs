use crate::xor::compare::xor_u8_cmp;
use std::marker::PhantomData;
use embre_crypt::xor::xor_bytes;

#[derive(Debug)]
pub struct XORResource<'a, T> {
    pub(super) resource: &'a [u8],
    pub(super) key: &'a [u8],
    pub(super) phantom_data: PhantomData<T>,
}
impl<'a, T> XORResource<'a, T> {
    pub const fn new(resource: &'a [u8], key: &'a [u8]) -> XORResource<'a, T> {
        XORResource {
            resource,
            key,
            phantom_data: PhantomData,
        }
    }
    /// Returns a Vec<u8> with the decrypted data.   
    pub fn to_plaintext_data(&self) -> Vec<u8> {
        xor_bytes(self.resource, self.key)
    }
    pub fn get_encrypted_slice(&self) -> &'a [u8] {
        self.resource
    }
    pub fn get_key(&self) -> &'a [u8] {
        self.key
    }
}
impl<T> PartialEq<Self> for XORResource<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.resource == other.resource && self.key == other.key
    }
}
impl<T> Eq for XORResource<'_, T> {}
impl<T> PartialEq<[u8]> for XORResource<'_, T> {
    fn eq(&self, other: &[u8]) -> bool {
        xor_u8_cmp(self.resource, self.key, other)
    }
}
impl<T> PartialEq<XORResource<'_, T>> for [u8] {
    fn eq(&self, other: &XORResource<'_, T>) -> bool {
        xor_u8_cmp(other.resource, other.key, self)
    }
}
impl<T> PartialEq<&[u8]> for XORResource<'_, T> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.eq(*other)
    }
}
impl<T> PartialEq<XORResource<'_, T>> for &[u8] {
    fn eq(&self, other: &XORResource<'_, T>) -> bool {
        other.eq(self)
    }
}
impl<T> PartialEq<Vec<u8>> for XORResource<'_, T> {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.eq(&other[..])
    }
}
impl<T> PartialEq<XORResource<'_, T>> for Vec<u8> {
    fn eq(&self, other: &XORResource<'_, T>) -> bool {
        other.eq(&self[..])
    }
}
