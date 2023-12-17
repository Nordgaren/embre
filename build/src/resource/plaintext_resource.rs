use crate::resource::{GetResourceName, Resource};
use crate::util::make_const_name;

pub(crate) struct PlaintextResource {
    pub resource_name: String,
    pub resource: Resource,
}

impl PlaintextResource {
    pub fn new(string_name: &str) -> PlaintextResource {
        PlaintextResource {
            resource_name: make_const_name(string_name),
            resource: Resource::new(string_name.as_bytes().to_vec(), usize::MAX),
        }
    }
}

impl GetResourceName for PlaintextResource {
    fn get_resource_name(&self) -> &String {
        &self.resource_name
    }
}
