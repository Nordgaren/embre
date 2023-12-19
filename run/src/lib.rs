use crate::xor_resource::XORResource;

pub mod xor_string;
mod pe_resource;
mod xor_resource;
pub mod xor_data;
pub(crate) mod util;
mod aes_string;
pub struct StringResource;
#[derive(Debug)]
pub struct DataResource;

pub trait EmbeddedResource {
    fn get_resource(self, offset: usize, len: usize) -> &'static [u8];
    fn get_xor_string(self, data_offset: usize, key_offset: usize, len: usize) -> XORResource<'static, StringResource>;
    fn get_xor_data(self, data_offset: usize, key_offset: usize, len: usize) -> XORResource<'static, DataResource>;
}


