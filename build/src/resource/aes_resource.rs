#![allow(unused)]
use crate::resource::xor_resource::XORResource;
use crate::resource::{GetResourceName, Resource};
use crate::util::{aes_encrypt_bytes, generate_random_bytes, make_const_name, xor_bytes};

pub struct AESResource {
    pub resource_name: String,
    pub(crate) encrypted_resource: Resource,
    pub(crate) key: Resource,
    pub(crate) iv: Resource,
}

impl AESResource {
    pub fn new(
        resource_name: &str,
        string_name: &[u8],
        key: Option<Vec<u8>>,
        iv: Option<Vec<u8>>,
    ) -> AESResource {
        let key = match key {
            Some(v) => Resource::new(v),
            // @TODO: Figure out a way to get the key size from the crypter.
            None => Resource::new(generate_random_bytes(32)),
        };
        let iv = match iv {
            Some(v) => Resource::new(v),
            // @TODO: Figure out a way to get the iv size from the crypter.
            None => Resource::new(generate_random_bytes(16)),
        };

        AESResource {
            resource_name: make_const_name(resource_name),
            encrypted_resource: Resource::new(aes_encrypt_bytes(
                string_name,
                &key.bytes[..],
                &iv.bytes[..],
            )),
            key,
            iv,
        }
    }
    pub fn named_str(string_name: &str, string: &str) -> AESResource {
        Self::new(string_name, string.as_bytes(), None, None)
    }
    pub fn named(string_name: &str, resource: &[u8]) -> AESResource {
        Self::new(string_name, resource, None, None)
    }
    pub fn from_str(string_name: &str, key: Option<Vec<u8>>, iv: Option<Vec<u8>>) -> AESResource {
        Self::new(string_name, string_name.as_bytes(), key, iv)
    }
}

impl GetResourceName for AESResource {
    fn get_resource_name(&self) -> &String {
        &self.resource_name
    }
}
