use crate::aes::aes_data::AESData;
use crate::aes::aes_resource::AESResource;
use crate::{util, StringResource};
use embre_crypt::aes::AESCrypter;
use std::ffi::{CStr, CString, NulError};
use std::fmt::Display;
use std::string::FromUtf8Error;
use widestring::U16CStr;

pub type AESString<'a> = AESResource<'a, StringResource>;
impl<'a> AESString<'a> {
    // This returns the original plaintext version of the string in a new String
    pub fn to_plaintext_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(
            self.to_plaintext_data()
                .expect("Could not decrypt aes resource for plaintext string."),
        )
    }
    // This returns the original plaintext version of the string in a new null terminated CString
    pub fn to_plaintext_c_string(&self) -> Result<CString, NulError> {
        CString::new(
            self.to_plaintext_data()
                .expect("Could not decrypt aes resource for plaintext string."),
        )
    }
}
impl From<AESData<'static>> for AESString<'static> {
    fn from(data: AESData<'_>) -> AESString<'_> {
        AESString {
            resource: data.resource,
            key: data.key,
            iv: data.iv,
            crypter: data.crypter,
            phantom_data: Default::default(),
        }
    }
}
impl<'a> Display for AESString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        util::common_string_fmt(f, self.to_plaintext_string())
    }
}
// Eq for utf-8 or ascii strings
impl PartialEq<&str> for AESString<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.eq(other.as_bytes())
    }
}
impl PartialEq<String> for AESString<'_> {
    fn eq(&self, other: &String) -> bool {
        self.eq(other.as_bytes())
    }
}
impl PartialEq<CStr> for AESString<'_> {
    fn eq(&self, other: &CStr) -> bool {
        self.eq(other.to_bytes())
    }
}
// EQ for wide strings
impl PartialEq<[u16]> for AESString<'_> {
    fn eq(&self, other: &[u16]) -> bool {
        let len = other.len().checked_mul(2).unwrap();
        let ptr: *const u8 = other.as_ptr().cast();
        let other = unsafe { std::slice::from_raw_parts(ptr, len) };
        self.crypter
            .aes_compare_string(self.resource, self.key, self.iv, other)
    }
}
impl PartialEq<&[u16]> for AESString<'_> {
    fn eq(&self, other: &&[u16]) -> bool {
        self.eq(*other)
    }
}
impl PartialEq<U16CStr> for AESString<'_> {
    fn eq(&self, other: &U16CStr) -> bool {
        self.eq(other.as_slice())
    }
}
