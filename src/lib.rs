#![doc = include_str!("../README.md")]
#![allow(unused)]
pub use embre_macro::include_aes_bytes;
pub use embre_macro::include_aes_str;
pub use embre_macro::include_xor_bytes;
pub use embre_macro::include_xor_str;

pub mod aes;
pub mod embedded_resource;
pub(crate) mod util;
pub mod xor;
#[derive(Debug)]
pub struct StringResource;
#[derive(Debug)]
pub struct DataResource;
