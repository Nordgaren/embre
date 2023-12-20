use crate::xor::xor_resource::XORResource;
use crate::util;
use core::ffi::CStr;
use std::ffi::{CString, NulError};
use std::fmt::Display;
use std::string::FromUtf8Error;
use widestring::U16CStr;
use crate::xor::xor_data::XORData;

pub type XORString<'a> = XORResource<'a, String>;
impl<'a> XORString<'a> {
    // This returns the original plaintext version of the string in a new String
    pub fn to_plaintext_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.to_plaintext_data())
    }
    // This returns the original plaintext version of the string in a new null terminated CString
    pub fn to_plaintext_c_string(&self) -> Result<CString, NulError> {
        CString::new(self.to_plaintext_data())
    }
}
impl From<XORData<'static>> for  XORString<'static> {
    fn from(data: XORData<'_>) -> XORString<'_> {
        XORString {
            resource: data.resource,
            key: data.key,
            phantom_data: Default::default(),
        }
    }
}
impl<'a> Display for XORString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        util::common_string_fmt(f, self.to_plaintext_string())
    }
}
// Eq for utf-8 or ascii strings
impl PartialEq<&str> for XORString<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.eq(other.as_bytes())
    }
}
impl PartialEq<String> for XORString<'_> {
    fn eq(&self, other: &String) -> bool {
        self.eq(other.as_bytes())
    }
}
impl PartialEq<CStr> for XORString<'_> {
    fn eq(&self, other: &CStr) -> bool {
        self.eq(other.to_bytes())
    }
}
// EQ for wide strings
impl PartialEq<[u16]> for XORString<'_> {
    fn eq(&self, other: &[u16]) -> bool {
        util::xor_w_str_cmp(self.resource, self.key, other)
    }
}
impl PartialEq<&[u16]> for XORString<'_> {
    fn eq(&self, other: &&[u16]) -> bool {
        self.eq(*other)
    }
}
impl PartialEq<U16CStr> for XORString<'_> {
    fn eq(&self, other: &U16CStr) -> bool {
        self.eq(other.as_slice())
    }
}
