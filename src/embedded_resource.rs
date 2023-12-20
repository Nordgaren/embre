#![allow(unused)]
use crate::aes::aes_data::AESData;
use crate::aes::aes_string::AESString;
use crate::xor::xor_data::XORData;
use crate::xor::xor_string::XORString;
use crate::{DataResource, StringResource};

// For future
pub trait EmbeddedResource {
    fn get_resource(self, offset: usize, len: usize) -> &'static [u8];
    fn get_xor_string(
        self,
        data_offset: usize,
        key_offset: usize,
        len: usize,
    ) -> XORString<'static>;
    fn get_xor_data(self, data_offset: usize, key_offset: usize, len: usize) -> XORData<'static>;
    fn get_aes_string(
        self,
        data_offset: usize,
        key_offset: usize,
        len: usize,
    ) -> AESString<'static>;
    fn get_aes_data(self, data_offset: usize, key_offset: usize, len: usize) -> AESData<'static>;
}
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
    ) -> XORString<'static> {
        todo!()
    }
    fn get_xor_data(self, data_offset: usize, key_offset: usize, len: usize) -> XORData<'static> {
        todo!()
    }

    fn get_aes_string(
        self,
        data_offset: usize,
        key_offset: usize,
        len: usize,
    ) -> AESString<'static> {
        todo!()
    }

    fn get_aes_data(self, data_offset: usize, key_offset: usize, len: usize) -> AESData<'static> {
        todo!()
    }
}
