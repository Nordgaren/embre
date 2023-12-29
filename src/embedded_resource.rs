#![allow(unused)]

use crate::aes::aes_data::AESData;
use crate::aes::aes_string::AESString;
use crate::xor::xor_data::XORData;
use crate::xor::xor_string::XORString;
use crate::{DataResource, StringResource};

pub trait EmbeddedResource {
    fn get_resource(&self) -> Option<&'static [u8]>;
    fn get_xor_string(
        &self,
        string_offset: usize,
        key_offset: usize,
        len: usize,
    ) -> XORString<'static> {
        self.get_xor_data(string_offset, key_offset, len).into()
    }
    fn get_xor_data(&self, data_offset: usize, key_offset: usize, len: usize) -> XORData<'static> {
        let resource = self.get_resource().expect("Could not find static resource");
        let data = &resource[data_offset..];
        let key = &resource[key_offset..];
        XORData::new(&data[..len], &key[..len])
    }
    fn get_aes_string(
        &self,
        string_offset: usize,
        key_offset: usize,
        iv_offset: Option<usize>,
        len: usize,
    ) -> AESString<'static> {
        self.get_aes_data(string_offset, key_offset, iv_offset, len)
            .into()
    }
    fn get_aes_data(
        &self,
        data_offset: usize,
        key_offset: usize,
        iv_offset: Option<usize>,
        len: usize,
    ) -> AESData<'static> {
        let resource = self.get_resource().expect("Could not find static resource");
        let data = &resource[data_offset..];
        let key = &resource[key_offset..];
        let iv = iv_offset.map(|iv_offset| &resource[iv_offset..16]);
        AESData::new(&data[..len], &key[..32], iv)
    }
}

#[allow(non_snake_case)]
#[cfg(feature = "DefaultPEResource")]
pub mod default {
    use crate::embedded_resource::EmbeddedResource;

    pub struct PEResource {
        category_id: u32,
        resource_id: u32,
    }

    impl PEResource {
        pub fn new(category_id: u32, resource_id: u32) -> Self {
            PEResource {
                category_id,
                resource_id,
            }
        }
    }

    impl EmbeddedResource for PEResource {
        fn get_resource(&self) -> Option<&'static [u8]> {
            unsafe {
                let hModule = GetModuleHandleA(std::ptr::null::<u8>());
                if hModule == 0 {
                    return None;
                }

                let hRes = FindResourceA(
                    hModule,
                    self.resource_id as *const u8,
                    self.category_id as *const u8,
                );
                if hRes == 0 {
                    return None;
                }

                let hResLoad = LoadResource(hModule, hRes);
                if hResLoad == 0 {
                    return None;
                }

                let lpResLock = LockResource(hResLoad);
                if lpResLock == 0 {
                    return None;
                }

                let dwSize = SizeofResource(hModule, hRes);
                if dwSize == 0 {
                    return None;
                }

                Some(std::slice::from_raw_parts(
                    lpResLock as *const u8,
                    dwSize as usize,
                ))
            }
        }
    }

    #[link(name = "kernel32", kind = "raw-dylib")]
    #[allow(unused)]
    extern "system" {
        pub fn GetModuleHandleA(module_name: *const u8) -> usize;
        pub fn FindResourceA(hModule: usize, lpName: *const u8, lpType: *const u8) -> usize;
        pub fn LoadResource(hModule: usize, hResInfo: usize) -> usize;
        pub fn LockResource(hResData: usize) -> usize;
        pub fn SizeofResource(hModule: usize, hResInfo: usize) -> u32;
    }
}
