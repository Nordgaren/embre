#![allow(unused)]
use crate::make_const_name;
use crate::resource::{GetResourceName, Resource};

pub struct PlaintextResource {
    pub resource_name: String,
    pub(crate) resource: Resource,
}

impl PlaintextResource {
    pub fn new(string_name: &str) -> PlaintextResource {
        PlaintextResource {
            resource_name: make_const_name(string_name),
            resource: Resource::new(string_name.as_bytes().to_vec()),
        }
    }
}

impl GetResourceName for PlaintextResource {
    fn get_resource_name(&self) -> &String {
        &self.resource_name
    }
}
