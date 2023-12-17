use crate::{StringResource, DataResource, EmbeddedResource};
use crate::xor_resource::XORResource;

// const PE100:PEResource = PEResource(100);
pub struct PEResource {
    resource_id: u32,
    category_id: u32,
}

impl EmbeddedResource for PEResource {
    fn get_resource(self, offset: usize, len: usize) -> &'static [u8] {
        todo!()
    }
    fn get_xor_string(
        self,
        string_offset: usize,
        key_offset: usize,
        len: usize,
    ) -> XORResource<'static, StringResource> {
        todo!()
    }
    fn get_xor_data(
        self,
        data_offset: usize,
        key_offset: usize,
        len: usize,
    ) -> XORResource<'static, DataResource> {
        todo!()
    }
}
