use crate::xor::xor_resource::XORResource;
use crate::xor::xor_string::XORString;
use crate::DataResource;
use std::fmt::Display;
pub type XORData<'a> = XORResource<'a, DataResource>;

impl XORResource<'_, DataResource> {
    // implement XORData functionality
}
impl From<XORString<'static>> for XORData<'static> {
    fn from(string: XORString<'_>) -> XORData<'_> {
        XORData {
            resource: string.resource,
            key: string.key,
            phantom_data: Default::default(),
        }
    }
}
impl<'a> Display for XORData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.to_plaintext_data();
        write!(f, "{:?}", data)
    }
}