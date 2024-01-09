#![allow(unused)]

use crate::make_const_name;
use crate::resource::{GetResourceName, Resource};
use embre_utils::generate_random_bytes;
use std::str::FromStr;

pub struct XORResource {
    pub(crate) resource_name: String,
    pub(crate) encrypted_resource: Resource,
    pub(crate) key: Resource,
}

impl XORResource {
    pub fn new(resource_name: &str, plaintext_bytes: &[u8], key_bytes: Vec<u8>) -> XORResource {
        if plaintext_bytes.len() > key_bytes.len() {
            panic!("string and key length do not match {}", resource_name)
        }
        XORResource {
            resource_name: make_const_name(resource_name),
            encrypted_resource: Resource::new(embre_crypt::xor::xor_bytes(
                plaintext_bytes,
                &key_bytes[..],
            )),
            key: Resource::new(key_bytes),
        }
    }
    pub fn named(resource_name: &str, plaintext_bytes: &[u8]) -> XORResource {
        XORResource::new(
            resource_name,
            plaintext_bytes,
            generate_random_bytes(plaintext_bytes.len()),
        )
    }
    pub fn named_str(resource_name: &str, string: &str) -> XORResource {
        XORResource::new(
            resource_name,
            string.as_bytes(),
            generate_random_bytes(string.len()),
        )
    }
    pub fn from_str(string: &str) -> XORResource {
        XORResource::new(
            string,
            string.as_bytes(),
            generate_random_bytes(string.len()),
        )
    }
}

impl GetResourceName for XORResource {
    fn get_resource_name(&self) -> &String {
        &self.resource_name
    }
}
