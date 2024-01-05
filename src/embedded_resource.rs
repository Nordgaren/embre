#![allow(unused)]

use crate as embre;
#[cfg(feature = "aes")]
use crate::aes::aes_data::AESData;
#[cfg(feature = "aes")]
use crate::aes::aes_string::AESString;
use crate::xor::xor_data::XORData;
use crate::xor::xor_string::XORString;
use crate::{DataResource, StringResource};
#[cfg(feature = "aes")]
use embre_macro::include_str_aes;
use embre_macro::include_str_xor;

pub struct PEResource {
    pub category_id: u32,
    pub resource_id: u32,
}

impl PEResource {
    pub const fn new(category_id: u32, resource_id: u32) -> Self {
        PEResource {
            category_id,
            resource_id,
        }
    }
}

pub struct XOROffsets {
    pub data: usize,
    pub key: usize,
    pub len: usize,
}
impl XOROffsets {
    pub const fn new(data: usize, key: usize, len: usize) -> Self {
        XOROffsets { data, key, len }
    }
}
#[cfg(feature = "aes")]
pub struct AESOffsets {
    pub data: usize,
    pub key: usize,
    pub iv: Option<usize>,
    pub len: usize,
}
#[cfg(feature = "aes")]
impl AESOffsets {
    pub const fn new(data: usize, key: usize, iv: Option<usize>, len: usize) -> Self {
        AESOffsets { data, key, iv, len }
    }
}

pub trait EmbeddedResource {
    fn get_resource(&self) -> Option<&'static [u8]>;
}
pub trait EmbeddedXOR: EmbeddedResource {
    fn get_str(&self, offsets: XOROffsets) -> XORString<'static> {
        self.get_data(offsets).into()
    }
    fn get_data(&self, offsets: XOROffsets) -> XORData<'static> {
        let resource = self
            .get_resource()
            .unwrap_or_else(|| panic!("{}", include_str_xor!("Could not find static resource")));
        let data = &resource[offsets.data..];
        let key = &resource[offsets.key..];
        XORData::new(&data[..offsets.len], &key[..offsets.len])
    }
}
#[cfg(feature = "aes")]
pub trait EmbeddedAES: EmbeddedResource {
    fn get_str(&self, offsets: AESOffsets) -> AESString<'static> {
        self.get_data(offsets).into()
    }
    fn get_data(&self, offsets: AESOffsets) -> AESData<'static> {
        let resource = self
            .get_resource()
            .unwrap_or_else(|| panic!("{}", include_str_aes!("Could not find static resource")));
        let data = &resource[offsets.data..];
        let key = &resource[offsets.key..];
        let iv = offsets.iv.map(|iv_offset| &resource[iv_offset..16]);
        AESData::new(&data[..offsets.len], &key[..32], iv)
    }
}
#[allow(non_snake_case)]
#[cfg(feature = "default-pe-resource")]
pub mod default_impl {
    use crate::embedded_resource::EmbeddedAES;
    use crate::embedded_resource::EmbeddedResource;
    use crate::embedded_resource::EmbeddedXOR;
    use crate::embedded_resource::PEResource;

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

    impl EmbeddedXOR for PEResource {}
    #[cfg(feature = "aes")]
    impl EmbeddedAES for PEResource {}

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
