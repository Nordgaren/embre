use crate::aes::aes_resource::AESResource;
use crate::aes::aes_string::AESString;
use crate::DataResource;
use std::fmt::{Debug, Display};
use embre_macro::include_str_aes;
use crate as embre;

pub type AESData<'a> = AESResource<'a, DataResource>;

impl AESResource<'_, DataResource> {
    // implement XORData functionality
}

impl From<AESString<'static>> for AESData<'static> {
    fn from(string: AESString<'_>) -> AESData<'_> {
        AESData {
            resource: string.resource,
            key: string.key,
            iv: string.iv,
            crypter: string.crypter,
            phantom_data: Default::default(),
        }
    }
}

impl<'a> Display for AESData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_plaintext_data()
            .unwrap_or_else(|e| panic!("{} {e}", include_str_aes!("Could not decrypt AESData")))
            .fmt(f)
    }
}
