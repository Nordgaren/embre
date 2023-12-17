use crate::resource::{GetResourceName, Resource};
use crate::util::{aes_encrypt_bytes, make_const_name};

pub(crate) struct AESResource {
    pub resource_name: String,
    pub encrypted_resource: Resource,
    pub key: Resource,
    pub iv: Option<Resource>,
}

impl AESResource {
    pub fn new(string_name: &str, key_bytes: Vec<u8>, iv: Option<Vec<u8>>) -> AESResource {
        let iv = match iv {
            None => vec![],
            Some(iv) => iv,
        };

        AESResource {
            resource_name: make_const_name(string_name),
            encrypted_resource: Resource::new(aes_encrypt_bytes(
                string_name.as_bytes(),
                &key_bytes[..],
                &iv[..],
            ), usize::MAX),
            key: Resource::new(key_bytes, usize::MAX),
            iv: None,
        }
    }
}

impl GetResourceName for AESResource {
    fn get_resource_name(&self) -> &String {
        &self.resource_name
    }
}

