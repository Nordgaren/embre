pub mod aes_resource;
pub mod plaintext_resource;
pub mod xor_resource;

pub(crate) struct Resource {
    pub(super) bytes: Vec<u8>,
    pub(super) offset: usize,
}

impl Resource {
    pub fn new(bytes: Vec<u8>) -> Self {
        Resource {
            bytes,
            offset: usize::MAX,
        }
    }
}

pub trait GetResourceName {
    fn get_resource_name(&self) -> &String;
}
