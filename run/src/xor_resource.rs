use crate::util;
use std::marker::PhantomData;

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
    pub fn to_plaintext_data(&self) -> Vec<u8> {
        let mut chrs = self.resource.to_vec();
        for i in 0..chrs.len() {
            chrs[i] ^= self.key[i];
        }

        chrs
    }
}

impl<T> PartialEq<[u8]> for XORResource<'_, T> {
    fn eq(&self, other: &[u8]) -> bool {
        util::xor_u8_cmp(self.resource, self.key, other)
    }
}
impl<T> PartialEq<&[u8]> for XORResource<'_, T> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.eq(*other)
    }
}
impl<T> PartialEq<Vec<u8>> for XORResource<'_, T> {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.eq(&other[..])
    }
}
