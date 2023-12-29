#![allow(unused)]
use crate::resource::{GetResourceName, Resource};
use crate::util::{make_const_name, xor_bytes};

pub(crate) struct XORResource {
    pub resource_name: String,
    pub encrypted_resource: Resource,
    pub key: Resource,
}

impl XORResource {
    pub fn new(resource_name: &str, plaintext_bytes: &[u8], key_bytes: Vec<u8>) -> XORResource {
        if plaintext_bytes.len() > key_bytes.len() {
            panic!("string and key length do not match {}", resource_name)
        }
        XORResource {
            resource_name: make_const_name(resource_name),
            encrypted_resource: Resource::new(xor_bytes(plaintext_bytes, &key_bytes[..])),
            key: Resource::new(key_bytes),
        }
    }
    pub fn from_str(string: &str, key_bytes: Vec<u8>) -> XORResource {
        XORResource::new(string, string.as_bytes(), key_bytes)
    }
}

impl GetResourceName for XORResource {
    fn get_resource_name(&self) -> &String {
        &self.resource_name
    }
}
