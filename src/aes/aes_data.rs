use std::fmt::Display;
use crate::DataResource;
use crate::aes::aes_resource::AESResource;
use crate::aes::aes_string::AESString;
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
        let data = self.to_plaintext_data();
        write!(f, "{:?}", data)
    }
}