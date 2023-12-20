use crate::xor::xor_resource::XORResource;

mod pe_resource;
pub(crate) mod util;
pub mod aes;
pub mod xor;

pub struct StringResource;
#[derive(Debug)]
pub struct DataResource;

pub trait EmbeddedResource {
    fn get_resource(self, offset: usize, len: usize) -> &'static [u8];
    fn get_xor_string(self, data_offset: usize, key_offset: usize, len: usize) -> XORResource<'static, StringResource>;
    fn get_xor_data(self, data_offset: usize, key_offset: usize, len: usize) -> XORResource<'static, DataResource>;
}


